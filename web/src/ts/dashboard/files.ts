import { findGetParameter, getCookie } from "../util";
import { ALL_FILES_ENDPOINT } from "../config";
import { FileSystemEntryType, IAllFilesResponse } from "../server_types";
import { setup } from "../index";

export async function loadFiles() {
    await setup();
    let path = findGetParameter('path');

    let filesRequest = $.ajax({
        url: ALL_FILES_ENDPOINT,
        method: 'POST',
        data: {
            session_id: getCookie('sessionid'),
            folder: (path == null) ? "" : atob(path)
        }
    });

    filesRequest.then((e) => {
        let response =  <IAllFilesResponse> e;
        if(response.status != 200) {
            console.error(response.status);
        }

        const FS_ENTRY_TYPE_ARR = Object.values(FileSystemEntryType);
        response.files.sort((a, b) => FS_ENTRY_TYPE_ARR.indexOf(a.entry_type) - FS_ENTRY_TYPE_ARR.indexOf(b.entry_type));

        const FILE_BROWSER = document.getElementById("file-browser");

        response.files.forEach((fsEntry) => {
            let fsEntryDiv = document.createElement('div');
            fsEntryDiv.id = btoa(fsEntry.name);

            if(fsEntry.entry_type.toString() == 'Folder') {
                fsEntryDiv.classList.add('fs-folder');
                addFolderIcon(fsEntryDiv);
                fsEntryDiv.addEventListener('click', (_e) => {
                    let url = new URL(window.location.href);
                    let pathNew = (path != null) ? atob(path) + "/" + fsEntry.name : fsEntry.name; 
                    url.searchParams.set('path', btoa(pathNew));
                    window.location.href = url.toString(); 
                });
            } else if(fsEntry.entry_type.toString() == 'File') {
                fsEntryDiv.classList.add('fs-file');
                addFileIcon(fsEntryDiv);
            } else {
                fsEntryDiv.classList.add('fs-unsupported');
                addUnsupportedIcon(fsEntryDiv);
            }

            let fileNameTextElem = document.createElement('p');
            fileNameTextElem.innerHTML = fsEntry.name;

            fsEntryDiv.appendChild(fileNameTextElem);
            FILE_BROWSER.appendChild(fsEntryDiv);
        })
    });
}

function addFolderIcon(elem: HTMLElement) {

}

function addFileIcon(elem: HTMLElement) {

}

function addUnsupportedIcon(elem: HTMLElement) {

}