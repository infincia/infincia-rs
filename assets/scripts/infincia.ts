/* Infincia, Copyright 2011-2017 Stephen Oliver */

import "material-design-lite";

//import "Tokenize2";

import * as Highcharts from 'highcharts';

/* ********************************************************************************************
 *
 *  Models
 *
 * */

interface FromJSON<T> {
    fromJSON(json: string): T;
}

export class Stats {
    public cpu_freq: string;
    public mem_used: number;
    public mem_total: number;
    public disk_used: number;
    public disk_total: number;
    public os_version: string;
    public app_version: string;
    public app_workers: number;

    public swap_total: number;
    public swap_free: number;
    public mem_avail: number;
    public mem_buffers: number;
    public mem_cached: number;

    static fromJSON(json: string): Stats {
        let o = JSON.parse(json);
        return new Stats(
            o['cpu_freq'],
            o['mem_used'],
            o['mem_total'],
            o['disk_used'],
            o['disk_total'],
            o['os_version'],
            o['app_version'],
            o['app_workers'],
            o['swap_total'],
            o['swap_free'],
            o['mem_avail'],
            o['mem_buffers'],
            o['mem_cached'])
    }

    constructor(cpu_freq: string,
                mem_used: number,
                mem_total: number,
                disk_used: number,
                disk_total: number,
                os_version: string,
                app_version: string,
                app_workers: number,
                swap_total: number,
                swap_free: number,
                mem_avail: number,
                mem_buffers: number,
                mem_cached: number) {
        this.cpu_freq = cpu_freq;
        this.mem_used = mem_used;
        this.mem_total = mem_total;
        this.mem_avail = mem_avail;
        this.mem_buffers = mem_buffers;
        this.mem_cached = mem_cached;

        this.disk_used = disk_used;
        this.disk_total = disk_total;

        this.os_version = os_version;

        this.app_version = app_version;
        this.app_workers = app_workers;

        this.swap_total = swap_total;
        this.swap_free = swap_free;
    }
}

export class Post {
    public id: number;
    public title: string;
    public url: string;
    public description: string;
    public content: string;

    public published: boolean;
    public tags: Array<string>;

    static fromJSON(json: string): Post {
        let o = JSON.parse(json);
        return new Post(
            o['id'],
            o['title'],
            o['url'],
            o['description'],
            o['content'],
            o['published'],
            o['tags'])
    }

    constructor(id: number,
                title: string,
                url: string,
                description: string,
                content: string,
                published: boolean,
                tags: Array<string>) {
        this.id = id;
        this.title = title;
        this.url = url;
        this.description = description;
        this.content = content;
        this.published = published;
        this.tags = tags;
    }
}

/* ********************************************************************************************
 *
 *  Maintenance
 *
 * */


export function maintenance_switch(state: boolean) {
    console.log(`maintenance state: ${state}`);

}

/* ********************************************************************************************
 *
 *  Login
 *
 * */

export function login() {
    let emailField = <HTMLInputElement>document.getElementById('emailField');
    let passwordField = <HTMLInputElement>document.getElementById('passwordField');

    let email: string = emailField.value;
    let password: string = passwordField.value;

    if (email === '' || email === undefined) {
        emailField.focus();
        Dialog.error(new Error('User missing')).show().then(() => {

        }).catch((error: Error) => {
            console.log(`user missing promise rejected': ${error.message}`);
        });
        return false;
    }
    if (password === '' || password === undefined) {
        passwordField.focus();
        Dialog.error(new Error('Password missing')).show().then(() => {

        }).catch((error: Error) => {
            console.log(`password missing promise rejected': ${error.message}`);
        });
        return false;
    }

    let login_credentials = {
        email: email,
        password: password
    };

    Request.post('/admin/login', login_credentials).start().then(() => {
        window.location.href = '/admin';
    }).catch((error) => {
        console.log('login failed: ', error);
        return Dialog.error(error).show();
    });
}

/* ********************************************************************************************
 *
 *  Registration
 *
 * */

export function register() {
    let nameField = <HTMLInputElement>document.getElementById('nameField');
    let emailField = <HTMLInputElement>document.getElementById('emailField');
    let passwordField = <HTMLInputElement>document.getElementById('passwordField');
    let registrationKeyField = <HTMLInputElement>document.getElementById('registrationKeyField');

    let name = nameField.value;
    let email = emailField.value;
    let password = passwordField.value;
    let registration_key = registrationKeyField.value;

    if (registration_key === '' || registration_key === undefined) {
        registrationKeyField.focus();
        return false;
    }

    if (name === '' || name === undefined) {
        nameField.focus();
        return false;
    }

    if (email === '' || email === undefined) {
        emailField.focus();
        return false;
    }

    if (password === '' || password === undefined) {
        passwordField.focus();
        return false;
    }

    let registration_credentials = {
        name: name,
        email: email,
        password: password,
        registration_key: registration_key
    };

    Request.post('/admin/register', registration_credentials).start().then(() => {
        window.location.href = '/admin/login';
    }).catch((error) => {
        console.log('register failed: ', error);
        return Dialog.error(error).show();
    });
}




/* ********************************************************************************************
 *
 *  Post editing
 *
 * */

export function createPost() {
    let method = 'POST';

    savePost(-1, method);
}

export function updatePost(id: number) {
    let method = 'PUT';

    savePost(id, method);
}

export function savePost(id: number, method: string) {
    let publishedCheckbox = <HTMLInputElement>document.getElementById('publishedCheckbox');
    let titleField =  <HTMLInputElement>document.getElementById('titleField');
    let urlField = <HTMLInputElement>document.getElementById('urlField');
    let tagBox = <HTMLSelectElement>document.getElementById('tagBox');
    let postBox = <HTMLInputElement>document.getElementById('postBox');
    let descriptionBox = <HTMLInputElement>document.getElementById('descriptionBox');

    let published = publishedCheckbox.checked;
    let title = titleField.value;
    let url = urlField.value;

    let tags = tagBox.value;
    let content = postBox.value;
    let description = descriptionBox.value;

    if (title === '' || title === undefined) {
        titleField.focus();
        Dialog.error(new Error('Title missing')).show().then(() => {

        });
        return false;
    }
    if (url === '' || url === undefined) {
        urlField.focus();
        Dialog.error(new Error('URL missing')).show().then(() => {

        });
        return false;
    }
    if (content === '' || content === undefined) {
        postBox.focus();
        Dialog.error(new Error('Content missing')).show().then(() => {

        });
        return false;
    }
    if (description === '' || description === undefined) {
        descriptionBox.focus();
        Dialog.error(new Error('Description missing')).show().then(() => {

        });
        return false;
    }
    if (tags === '' || tags === undefined) {
        tagBox.focus();
        Dialog.error(new Error('Tags missing')).show().then(() => {

        });
        return false;
    }

    let post = new Post(id, title, url, description, content, published, tags.split(", "));

    let request = new Request('/admin/posts', method, post);

    request.start().then(() => {
        if (id === -1) {
            window.location.href =  '/admin/posts';
        }
    }).catch((error) => {
        console.log('save post failed: ', error);
        return Dialog.error(error).show();
    });
}

/* ********************************************************************************************
 *
 *  File handling
 *
 * */

export function uploadFile() {
    let fileForm = <HTMLFormElement>document.getElementById('file-upload-form');
    let fileInputField = <HTMLInputElement>document.getElementById('file-input');


    //let fileUploadStatusContainer = <HTMLInputElement>document.getElementById('file-upload-status-container');
    //let fileUploadStatus = <HTMLInputElement>document.getElementById('file-upload-status');
    let fileUploadProgressBar = <HTMLDivElement>document.getElementById('file-upload-progress-bar');

    if (fileInputField.files.length === 0) {
        Dialog.error(new Error('File missing')).show().then(() => {

        });
        return false;
    }

    let formData = new FormData(fileForm);

    let xhr = new XMLHttpRequest();

    xhr.onload = (event: Event) => {
        event.stopPropagation();

        fileUploadProgressBar.style.opacity = "1";
    };

    xhr.onabort = (event: Event) => {
        event.stopPropagation();

        fileUploadProgressBar.style.opacity = "0";

        Dialog.error(new Error('Upload aborted')).show().then(() => {

        });
    };

    /*
    xhr.onerror = (event: ErrorEvent) => {
        event.stopPropagation();

        Dialog.error(event.message).show().then(() => {

        }).catch((error: Error) => {

        });
    };*/

    xhr.upload.onprogress = (event: ProgressEvent) => {
        event.stopPropagation();

        let loaded: number = event.loaded;
        let total: number = event.total;

        let percent: string = ((loaded / total) * 100.0).toFixed(2);

        console.log(`upload progress: ${percent}`);

        componentHandler.upgradeElement(fileUploadProgressBar);

        let bar = <HTMLDivElement>fileUploadProgressBar.getElementsByClassName('progressbar').item(0);

        bar.style.width = percent + '%';

        //fileUploadStatus.innerHTML = `${percent}%`;
    };

    xhr.onreadystatechange = (event: Event) => {
        event.stopPropagation();
        
        try {
            console.log(`onreadystatechange: ${event}`);

            let readyState = xhr.readyState;
            let text = xhr.responseText;
            let status = xhr.status;

            console.log(`onreadystatechange state: ${readyState}`);
            console.log(`onreadystatechange text: ${text}`);
            console.log(`onreadystatechange status: ${status}`);


            if (readyState === 4 && status === 200) {
                fileUploadProgressBar.style.opacity = "0";

                let bar = <HTMLDivElement>fileUploadProgressBar.getElementsByClassName('progressbar').item(0);
                bar.style.width = 0 + '%';
            } else if (readyState === 4) {
                fileUploadProgressBar.style.opacity = "0";

                let bar = <HTMLDivElement>fileUploadProgressBar.getElementsByClassName('progressbar').item(0);
                bar.style.width = 0 + '%';

                Dialog.error(new Error(text)).show().then(() => {

                });
            }
        }
        catch(e) {
            fileUploadProgressBar.style.opacity = "0";

            let bar = <HTMLDivElement>fileUploadProgressBar.getElementsByClassName('progressbar').item(0);
            bar.style.width = 0 + '%';

            //fileUploadStatus.innerText = '';

            console.log(`upload error: ${e}`);

            Dialog.error(e.toString()).show().then(() => {

            });
        }
    };

    xhr.open('POST', '/admin/files', true);

    xhr.send(formData);

    return false;
}

/* ********************************************************************************************
 *
 *  Initialization
 *
 * */

export function initDashboard() {
    let maintenanceSwitch = <HTMLInputElement>document.getElementById('maintenance-switch');

    maintenanceSwitch.addEventListener('click', (event: Event) => {
        event.preventDefault();
        let state = maintenanceSwitch.checked;

        maintenance_switch(state);
    });

    let chart_container = <HTMLElement>document.getElementById('graph');

    let chart = Highcharts.chart({
                         chart: {
                             backgroundColor: 'rgba(0,0,0,0)',
                             type: 'area',
                             animation: true,
                             renderTo: chart_container,
                             events: {
                                 load: function() {
                                     updateDashboard(this);
                                 }
                             },
                         },
                         title: {
                             text: 'Server status',
                             style: {
                                 fontFamily: 'PT Sans Narrow',
                                 fontSize: '24px',
                                 fontWeight: 'bold'
                             }
                         },
                         xAxis: {
                             type: 'datetime',
                             tickPixelInterval: 100
                         },
                         yAxis: {
                             floor: 0,
                             ceiling: 100,
                             title: {
                                 text: 'Percent',
                                 style: {
                                     fontFamily: 'PT Sans Narrow',
                                     fontSize: '20px'
                                 }
                             },
                             plotLines: [{
                                 value: 0,
                                 width: 1,
                                 color: '#808080'
                             }],
                             labels: {
                                 format: '{value}%',
                                 style: {
                                     fontFamily: 'PT Sans Narrow',
                                     fontSize: '18px'
                                 }
                             }
                         },
                         legend: {
                             enabled: true,
                             title: {
                                 style: {
                                     fontFamily: 'PT Sans Narrow',
                                     fontSize: '14px'
                                 }
                             }
                         },
                         exporting: {
                             enabled: false
                         },
                         series: [{
                             name: 'RAM',
                             id: 'RAM',
                             zIndex: 2,
                             data: (() => {
                                 // generate an array of random data
                                 let data = [];
                                 let time = (new Date()).getTime();

                                 for (let i = -19; i <= 0; i++) {
                                     data.push({
                                                   x: time + i * 1000,
                                                   y: 0
                                               });
                                 }
                                 return data;
                             })()
                         }, {
                             name: 'CPU',
                             id: 'CPU',
                             zIndex: 3,
                             data: (() => {
                                 // generate an array of random data
                                 let data = [];
                                 let time = (new Date()).getTime();

                                 for (let i = -19; i <= 0; i++) {
                                     data.push({
                                                   x: time + i * 1000,
                                                   y: 0
                                               });
                                 }
                                 return data;
                             })()
                         }, {
                             name: 'DISK',
                             id: 'DISK',
                             zIndex: 1,
                             data: (() => {
                                 // generate an array of random data
                                 let data = [];
                                 let time = (new Date()).getTime();

                                 for (let i = -19; i <= 0; i++) {
                                     data.push({
                                                   x: time + i * 1000,
                                                   y: 0
                                               });
                                 }
                                 return data;
                             })()
                         }]
                     });


    window['timerID'] = setInterval(() => {
        updateDashboard(chart);
    }, 10000);

    window['chart'] = chart;
}

function updateDashboard(chart: Highcharts.ChartObject) {
    Request.get('/admin/stats').start(Stats).then((stats) => {
        let x = (new Date()).getTime(); // current time
        let mem_used: number = stats.mem_used;
        let mem_total: number = stats.mem_total;
        //let mem_avail: number = stats.mem_avail;
        //let mem_buffers: number = stats.mem_buffers;
        //let mem_cached: number = stats.mem_cached;

        let swap_free: number = stats.swap_free;
        let swap_total: number = stats.swap_total;
        let swap_used = 0;

        if (swap_total != 0) {
            swap_used = (swap_total - swap_free);
        }

        let disk_used: number = stats.disk_used;
        let disk_total: number = stats.disk_total;

        let swap_percent: string = ((swap_used / swap_total) * 100.0).toFixed(2);

        let mem_percent: string = ((mem_used / mem_total) * 100.0).toFixed(2);
        let disk_percent: string = ((disk_used / disk_total) * 100.0).toFixed(2);

        //let cpu_chart = chart.get('CPU') as Highcharts.SeriesObject;
        let disk_chart = chart.get('DISK') as Highcharts.SeriesObject;
        let ram_chart = chart.get('RAM') as Highcharts.SeriesObject;

        //cpu_chart.addPoint([x, stats.cpu_freq], true, true);
        ram_chart.addPoint([x, (stats.mem_used / stats.mem_total) * 100.0], true, true);
        disk_chart.addPoint([x, (stats.disk_used / stats.disk_total) * 100.0], true, true);

        let cpuField = <HTMLInputElement>document.getElementById('cpufield');
        cpuField.innerText = `${stats.cpu_freq}Mhz`;

        let memUsedField = <HTMLInputElement>document.getElementById('memusedfield');
        memUsedField.innerText = `${mem_percent}%`;

        let diskUsedField = <HTMLInputElement>document.getElementById('diskusedfield');
        diskUsedField.innerText = `${disk_percent}%`;

        let appVersionField = <HTMLInputElement>document.getElementById('application-version');
        appVersionField.innerText = stats.app_version;

        let appWorkersField = <HTMLInputElement>document.getElementById('application-workers');
        appWorkersField.innerText = stats.app_workers.toString();

        let osVersionField = <HTMLInputElement>document.getElementById('os-version');
        osVersionField.innerText = stats.os_version;

        let swapField = <HTMLInputElement>document.getElementById('swapusedfield');
        if (swap_total != 0) {
            swapField.innerText = `${swap_percent}%`;
        } else {
            swapField.innerText = `N/A`;
        }

    }).catch((error) => {
        console.log('get stats failed: ', error);
    });
}

export function initPostList() {

    document.getElementById('add-post-button').addEventListener('click', (event: Event) => {
        event.preventDefault();
        window.location.href = '/admin/posts/0';
    });

    let viewPostButtons = document.getElementsByClassName('view-post-button') as NodeListOf<Element>;

    let editPostButtons = document.getElementsByClassName('edit-post-button') as NodeListOf<Element>;


    let deletePostButtons = document.getElementsByClassName('delete-post-button') as NodeListOf<Element>;

    Array.from(viewPostButtons).forEach((element) => {
        element.addEventListener('click', (event: Event) => {
            event.preventDefault();
            let post_url = (<HTMLElement>event.currentTarget).dataset['postUrl'];

            window.location.href = `/blog/${post_url}`;
        });
    });

    Array.from(editPostButtons).forEach((element) => {
        element.addEventListener('click', (event: Event) => {
            let post_id = (<HTMLElement>event.currentTarget).dataset['postId'];

            window.location.href = `/admin/posts/${post_id}`;
        });
    });

    Array.from(deletePostButtons).forEach((element) => {
        element.addEventListener('click', (event: Event) => {
            let post_id = Number((<HTMLElement>event.currentTarget).dataset['postId']);
            let post_row = (<HTMLElement>event.currentTarget).parentElement.parentElement;

            Dialog.confirm('Delete post?', post_id.toString()).show().then(() => {
                return Request._delete(`/admin/posts/${post_id}`).start();
            }).then(() => {
                post_row.remove();
            }).catch((error: Error) => {
                console.log('delete post failed: ', error);
                return Dialog.error(error).show();
            });
        });
    });
}

export function initPostEdit(new_post: boolean) {
    let tagBox = document.getElementById('tagBox');
    let initial_tags = tagBox.dataset['initialTags'];
    let initial_tag_list = initial_tags.split(' ');

    if (new_post) {
        document.getElementById('create-post-button').addEventListener('click', (event: Event) => {
            event.preventDefault();
            createPost();
        });
    } else {
        document.getElementById('update-post-button').addEventListener('click', (event: Event) => {
            event.preventDefault();

            let post_id = Number((<HTMLElement>event.currentTarget).dataset['postId']);

            updatePost(post_id);
        });
    }

    /*
    tagBox.tokenize2({
                         dataSource: '/admin/posts/tags',
                         tokensAllowCustom: true,
                         delimiter: [',', '-', ' ']
                     });

    initial_tag_list.forEach((tag: string, index: Number, array: Array<string>) => {
        tagBox.tokenize2()
            .trigger('tokenize:tokens:add',
                [tag, tag, true]);
    });*/

    // fix label
    let tokenFields = document.getElementsByClassName('tokenize') as NodeListOf<Element>;

    Array.from(tokenFields).forEach((element) => {
        event.preventDefault();
        element.classList.add('mdl-textfield__input');
        if (initial_tag_list.length > 0) {
            element.parentElement.classList.add('is-dirty');
        }
    });
}

export function initRegister() {
    document.getElementById('register-user-button').addEventListener('click', (event: Event) => {
        event.preventDefault();
        register();
    });
}

export function initLogin() {
    document.getElementById('login-button').addEventListener('click', (event: Event) => {
        event.preventDefault();
        login();
    });
}

export function initUserList() {
    document.getElementById('add-user-button').addEventListener('click', (event: Event) => {
        event.preventDefault();
        window.location.href = '/admin/register';
    });

    let deleteUserButtons = document.getElementsByClassName('delete-user-button') as NodeListOf<Element>;

    Array.from(deleteUserButtons).forEach((element) => {
        element.addEventListener('click', (event: Event) => {
            event.preventDefault();
            let user_name = (<HTMLElement>event.currentTarget).dataset['userName'];
            let user_row = (<HTMLElement>event.currentTarget).parentElement;

            Dialog.confirm('Delete user?', user_name).show().then(() => {
                return Request._delete(`/admin/users/${user_name}`).start();
            }).then(() => {
                user_row.remove();
            }).catch((error: Error) => {
                console.log('delete user failed: ', error);
                return Dialog.error(error).show();
            });
        });
    });
}

export function initFileList() {
    let fileUploadForm = <HTMLFormElement>document.getElementById('file-upload-form');

    let fileNameField = <HTMLInputElement>document.getElementById('file-name-field');
    let uploadFileButton = <HTMLButtonElement>document.getElementById('upload-file-button');
    let fileInputField = <HTMLInputElement>document.getElementById('file-input');

    let downloadFileButtons = document.getElementsByClassName('download-file-button') as NodeListOf<Element>;
    let infoFileButtons = document.getElementsByClassName('info-file-button') as NodeListOf<Element>;
    let refreshFileButtons = document.getElementsByClassName('refresh-file-button') as NodeListOf<Element>;
    let deleteFileButtons = document.getElementsByClassName('delete-file-button') as NodeListOf<Element>;


    fileNameField.value = '';
    fileNameField.parentElement.classList.remove('is-focused');
    fileNameField.parentElement.classList.remove('is-dirty');

    fileUploadForm.addEventListener('submit', (event: Event) => {
        event.preventDefault();
        uploadFile();
    });

    fileInputField.addEventListener('change', (event: Event) => {
        event.preventDefault();

        fileNameField.value = fileInputField.files[0].name;

        fileNameField.parentElement.classList.add('is-focused');
        fileNameField.parentElement.classList.add('is-dirty');
    });


    uploadFileButton.addEventListener('click', (event: Event) => {
        event.preventDefault();
        uploadFile();
    });

    Array.from(downloadFileButtons).forEach((element) => {
        element.addEventListener('click', (event: Event) => {
            event.preventDefault();
            let file_name = (<HTMLElement>event.currentTarget).dataset['fileName'];

            window.location.href = `/files/download/${file_name}`;
        });
    });

    Array.from(infoFileButtons).forEach((element) => {
        element.addEventListener('click', (event: Event) => {
            event.preventDefault();
            let file_name = (<HTMLElement>event.currentTarget).dataset['fileName'];

            window.location.href = `/files/info/${file_name}`;
        });
    });

    Array.from(refreshFileButtons).forEach((element) => {
        element.addEventListener('click', (event: Event) => {
            event.preventDefault();
            let file_name = (<HTMLElement>event.currentTarget).dataset['fileName'];

            Dialog.confirm('Refresh metadata?', file_name).show().then(() => {
                return Request.post(`/admin/files/refresh/${file_name}`).start();
            }).then(() => {
                window.location.href = '/admin/files';
            }).catch((error: Error) => {
                console.log('refresh file failed: ', error);
                return Dialog.error(error).show();
            });
        });
    });

    Array.from(deleteFileButtons).forEach((element) => {
        element.addEventListener('click', (event: Event) => {
            event.stopPropagation();
            let file_name = (<HTMLElement>event.currentTarget).dataset['fileName'];
            let file_card = (<HTMLElement>event.currentTarget).parentElement.parentElement;

            Dialog.confirm('Delete file?', file_name).show().then(() => {
                return Request._delete(`/admin/files/${file_name}`).start();
            }).then(() => {
                file_card.remove();
            }).catch((error: Error) => {
                console.log('delete file failed: ', error);
                return Dialog.error(error).show();
            });
        });
    });
}

/* ********************************************************************************************
*
*  HTTP Requests
*
* */

export class Request {
    method: string;
    url: string;
    body?: object;

    constructor(method: string,
                url: string,
                body?: object) {
        this.method = method;
        this.url = url;
        this.body = body;
    }


    static get(url: string): Request {
        return new Request('GET', url);
    }

    static _delete(url: string): Request {
        return new Request('DELETE', url);
    }

    static post(url: string, body?: Object): Request {
        return new Request('POST', url, body);
    }

    public start<T>(cls?: FromJSON<T>): Promise<T> {
        return new Promise<any>(
            (resolve, reject) => {
                const request = new XMLHttpRequest();

                request.onload = () => {

                    if (request.status === 200) {
                        if (cls) {
                            let reply = cls.fromJSON(request.responseText);

                            resolve(reply);
                        } else {
                            resolve();
                        }
                    } else {
                        reject(new Error(request.statusText));
                    }
                };
                request.onerror = () => {
                    reject(new Error(`XMLHttpRequest Error: ${request.statusText}`));
                };

                request.open(this.method, this.url);

                if (this.body) {
                    let post_data = JSON.stringify(this.body);

                    request.setRequestHeader('Content-Type', 'application/json');
                    request.send(post_data);
                } else {
                    request.send();
                }
            });
    }
}

export class Dialog {
    id: string;
    title: string;
    text: string;
    useOK: boolean;
    useNo: boolean;
    useCancel: boolean;
    useYes: boolean;

    constructor(title: string,
                text: string,
                useOK: boolean,
                useNo: boolean,
                useCancel: boolean,
                useYes: boolean) {
        let uid = Math.random().toString(36).substring(2, 15);

        this.id = `modal-dialog-${uid}`;
        this.title = title;
        this.text = text;
        this.useOK = useOK;
        this.useNo = useNo;
        this.useCancel = useCancel;
        this.useYes = useYes;
    }

    static confirm(title: string, text: string): Dialog {
        return new Dialog(title, text, false, true, false, true);
    }

    static error(error: Error): Dialog {
        return new Dialog(error.name, error.message, true, false, false, false);
    }

    public show(): Promise<any> {
        return new Promise<any>((resolve, reject) => {

            let oldDialogs = document.getElementsByClassName('modal-dialog-container') as NodeListOf<Element>;

            Array.from(oldDialogs).forEach((element) => {
                element.remove();
            });

            let dialogContainer = document.createElement('div');
            dialogContainer.className = 'modal-dialog-container';
            dialogContainer.id = `${this.id}-container`;
            dialogContainer.style.opacity = "1";

            document.body.appendChild(dialogContainer);

            dialogContainer.innerHTML = `
                <div id='${this.id}-content' class='modal-dialog-content mdl-card mdl-shadow--16dp'>
                    <h5 id='${this.id}-title' class='modal-dialog-title'>${this.title}</h5>
                    <p id='${this.id}-text' class='modal-dialog-text'>${this.text}</p>
                    <div id='${this.id}-buttons' class='modal-dialog-buttons mdl-card__actions'>
                        <button id='${this.id}-button-no' class='modal-dialog-button modal-dialog-button-no mdl-button mdl-js-button mdl-js-ripple-effect'>No</button>
                        <button id='${this.id}-button-cancel' class='modal-dialog-button modal-dialog-button-cancel mdl-button mdl-js-button mdl-js-ripple-effect'>Cancel</button>
                        <button id='${this.id}-button-yes' class='modal-dialog-button modal-dialog-button-yes mdl-button mdl-js-button mdl-js-ripple-effect'>Yes</button>
                        <button id='${this.id}-button-ok' class='modal-dialog-button modal-dialog-button-ok mdl-button mdl-js-button mdl-js-ripple-effect'>OK</button>
                    </div>
                </div>`;

            let noButton = document.getElementById(`${this.id}-button-no`);
            let cancelButton= document.getElementById(`${this.id}-button-cancel`);
            let yesButton = document.getElementById(`${this.id}-button-yes`);
            let okButton= document.getElementById(`${this.id}-button-ok`);

            noButton.style.display = "none";
            cancelButton.style.display = "none";
            yesButton.style.display = "none";
            okButton.style.display = "none";

            //let buttons = document.getElementById(`${this.id}-buttons`);
            //let dialogContent = dialogContainer.getElementsByClassName(`${this.id}-content`);
            //let dialogTitle = dialogContainer.getElementsByClassName(`${this.id}-buttons`);
            //let dialogText= dialogContainer.getElementsByClassName(`${this.id}-buttons`);

            if (this.useNo) {
                noButton.style.display = "inline-block";
                noButton.addEventListener('click', (event: Event) => {
                    event.stopPropagation();

                    console.log('dialog clicked no');

                    dialogContainer.style.opacity = "0";
                    dialogContainer.remove();
                    reject(new Error(`no`));
                });
            }

            if (this.useCancel) {
                cancelButton.style.display = "inline-block";
                cancelButton.addEventListener('click', (event: Event) => {
                    event.stopPropagation();

                    console.log('dialog clicked cancel');

                    dialogContainer.style.opacity = "0";
                    dialogContainer.remove();
                    reject(new Error(`cancel`));
                });
            }

            if (this.useYes) {
                yesButton.style.display = "inline-block";
                yesButton.addEventListener('click', (event: Event) => {
                    event.stopPropagation();

                    console.log('dialog clicked yes');

                    dialogContainer.style.opacity = "0";
                    dialogContainer.remove();
                    resolve();
                });
            }

            if (this.useOK) {
                okButton.style.display = "inline-block";
                okButton.addEventListener('click', (event: Event) => {
                    event.stopPropagation();

                    console.log('dialog clicked ok');

                    dialogContainer.style.opacity = "0";
                    dialogContainer.remove();
                    resolve();
                });
            }
        });
    }
}
