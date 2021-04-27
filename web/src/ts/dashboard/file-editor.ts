import { setup                } from "../index";
import { findGetParameter, getCookie, htmlEscape } from "../util";
import { FILE_CONTENT_ENDPOINT, FILE_SAVE_ENDPOINT } from "../config";
import { IFileContentResponse } from "../server_types";

export async function setupFileEditor() {
    await setup();

    let file = findGetParameter('file');
    if(file == null) {
        window.location.href = "files.html";
    }

    document.getElementById('path').innerHTML = atob(file);
    let getFileContentRequest = $.ajax({
        url: FILE_CONTENT_ENDPOINT,
        method: 'POST',
        data: {
            session_id: getCookie('sessionid'),
            filename: atob(file)
        }
    });

    getFileContentRequest.then((e) => {
        let response = <IFileContentResponse> e;
        if(response.status != 200) {
            console.error(response.status);
            return;
        }

        document.getElementById('file-editor-view').innerHTML = response.content;
        document.execCommand('defaultParagraphSeparator', false, "\\n");
        document.addEventListener('keydown', (e) => {
            if(e.key == 'Enter') {
                e.preventDefault();
                document.execCommand('insertHTML', false, '<br>');
                return false;
            }
        })
    });

    document.getElementById('saveBtn').addEventListener("click", (_e) => {
        let text = document.getElementById('file-editor-view').innerText;
        let file = atob(findGetParameter('file'))
        $.ajax({
            url: FILE_SAVE_ENDPOINT,
            method: 'POST',
            data: {
                session_id: getCookie('sessionid'),
                filename: file,
                content: text
            }
        });

        let parts = file.split("/");
        if(parts.length == 1) {
            window.location.href = "files.html";
            return;
        }

        let folder: string;
        for(let i = 0; i < parts.length -1; i++) {
            folder += parts[i];
        }

        window.location.href = "files.html?path=" + btoa(folder); 
    });
}