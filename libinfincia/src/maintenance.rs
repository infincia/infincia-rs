/* Infincia, Copyright 2011-2017 Stephen Oliver */
/* Version 1.0 */

use ::dbmodels::NewRepo;

use ::error::Error;

use ::database::AdminDB;
use ::database::DB;


use futures::{Future, Stream};
use hyper::{Method, Request, Client, Response, Chunk};
use hyper::header::UserAgent;
use tokio_core::reactor::Core;

use serde_json::{from_slice};

pub fn maintenance() -> Result<(), Error> {
    let wait_30_minutes = ::std::time::Duration::from_secs(1800);
    let mut core = Core::new()?;

    let client = Client::configure()

        .connector(::hyper_tls::HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

    loop {
        let uri = "https://api.github.com/users/infincia/repos".parse()?;
        let agent = UserAgent::new("infincia-rs/0.1");

        let mut req = Request::new(Method::Get, uri);
        req.headers_mut().set(agent);

        let work = client.request(req).and_then(|response: Response| {
            response.body().concat2().and_then(move |body: Chunk| {
                let repo_list: Vec<NewRepo> = match from_slice(&body) {
                    Ok(r) => r,
                    Err(err) => {
                        println!("WARNING: github repo list could not be parsed: {}", err);
                        let st = String::from_utf8_lossy(&body);
                        println!("WARNING: actual response: {}", st);

                        return Ok(());
                    }
                };

                let db = match DB::get_connection() {
                    Ok(connection) => connection,
                    Err(err) => {
                        println!("WARNING: {}", err);
                        return Ok(());

                    }
                };

                if let Err(err) = db.update_repos(repo_list) {
                    println!("WARNING: {}", err);
                    return Ok(());
                }

                Ok(())
            })
        });

        core.run(work)?;

        ::std::thread::sleep(wait_30_minutes);
    }
}

/*
while PROCESS_GITHUB:
try:
r = requestsession.get(url)
repos = r.json()
repo_list = list()
for repo in repos:
repo_list.append(repo)
# change order the repos appear in the document so new stuff is at the top
repo_list.reverse()
infincia_database.update_repos(repos=repo_list)
except ConnectionError as e:
local_log.debug('Github: max retries exceeded')
except MaxRetryError as e:
local_log.debug('Github: max retries exceeded')
except Exception as e:
local_log.exception('Github: exception getting repos: %s', e)
time.sleep(1800)
*/

/*
from __future__ import division, absolute_import, print_function

import os
import json
import re
import time
from datetime import timedelta
import threading
import socket
import json
import errno

# pip packages
import six
import psutil
import redis
import requests
from requests.adapters import HTTPAdapter
from requests.packages.urllib3.poolmanager import PoolManager
from requests.exceptions import ConnectionError
from requests.packages.urllib3.exceptions import MaxRetryError
import ssl


# local packages
import infincia.database
import infincia.util

__all__ = []

# setup logging to console, auto DEBUG level in development
local_log = infincia.util.logger()

# get the master config from the util module
local_config = infincia.util.config()


UWSG_STATS_ADDRESS = local_config.get('UWSGI', 'stats_address')
UWSG_STATS_PORT = local_config.getint('UWSGI', 'stats_port')



GITHUB_USERNAME = local_config.get('Github', 'username')


# globals
PROCESS_STATS = True
PROCESS_GITHUB = True

infincia_database = infincia.database.Database()

REDIS_HOST = os.environ['REDIS_PORT_6379_TCP_ADDR']
if REDIS_HOST is None:
REDIS_HOST = 'localhost'
REDIS_PORT = os.environ['REDIS_PORT_6379_TCP_PORT']
if REDIS_PORT is None:
REDIS_PORT = '6379'

infincia_redis    = redis.StrictRedis(host=REDIS_HOST, port=int(REDIS_PORT), db=0)




class RequestAdapter(HTTPAdapter):
def init_poolmanager(self, connections, maxsize, block):
self.poolmanager = PoolManager(num_pools=connections,
maxsize=maxsize,
block=block,
ssl_version=ssl.PROTOCOL_TLSv1)

requestsession = requests.Session()
requestsession.mount('https://', RequestAdapter())





def stats():
while PROCESS_STATS:
try:
buffer = []
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.settimeout(3)
s.connect((UWSG_STATS_ADDRESS, UWSG_STATS_PORT))

while True:
data = s.recv(4096)
if len(data) < 1:
break
buffer.append(data.decode('utf8'))

raw = json.loads(''.join(buffer))

stats = {}
stats['cpu'] = psutil.cpu_percent()
stats['memused'] = psutil.virtual_memory().percent
stats['swapused'] = psutil.swap_memory().percent
stats['diskused'] = psutil.disk_usage('/').percent
stats['numcpu'] = psutil.cpu_count()
stats['uwsgiversion'] = raw['version']
stats['uwsgiload'] = raw['load']
stats['uwsgipid'] = raw['pid']
stats['applicationrelease'] = 'dev'
stats['maintenance_mode'] = infincia.util.maintenance_mode()
with open('/proc/uptime', 'r') as f:
uptime_seconds = float(f.readline().split()[0])
days, remainder = divmod(timedelta(seconds=uptime_seconds).total_seconds(), 86400)
hours, remainder = divmod(remainder, 3600)
minutes, seconds = divmod(remainder, 60)

duration_formatted = '%d days, %d:%02d:%02d' % (days, hours, minutes, seconds)
stats['uptime'] = duration_formatted
infincia_redis.publish('api', json.dumps(stats))
except IOError as e:
if e.errno == errno.ECONNREFUSED:
local_log.error('Stats: connection refused')
if e.errno == errno.ECONNRESET:
local_log.error('Stats: connection reset')
if e.errno == errno.EINTR:
break
except UnicodeDecodeError as e:
local_log.error('Unicode error decoding buffer')
except Exception as e:
local_log.exception('Stats: unknown failure')
finally:
s.close()
time.sleep(1)




def cache():
while PROCESS_GITHUB:
try:
url = 'https://api.github.com/users/%s/repos' % GITHUB_USERNAME
r = requestsession.get(url)
repos = r.json()
repo_list = list()
for repo in repos:
repo_list.append(repo)
# change order the repos appear in the document so new stuff is at the top
repo_list.reverse()
infincia_database.update_repos(repos=repo_list)
except ConnectionError as e:
local_log.debug('Github: max retries exceeded')
except MaxRetryError as e:
local_log.debug('Github: max retries exceeded')
except Exception as e:
local_log.exception('Github: exception getting repos: %s', e)
time.sleep(1800)



statsthread = threading.Thread(target=stats)
cachethread = threading.Thread(target=cache)


threads = []

threads.append(statsthread)
threads.append(cachethread)

local_log.debug('Maintenance: worker starting')

for thread in threads:
thread.start()

for thread in threads:
thread.join()

local_log.debug('Maintenance: worker quitting')
*/