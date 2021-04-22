import { SESSION_ENDPOINT   } from './config';
import { getCookie          } from './util';
import { ISessionResponse   } from './server_types';

export async function isLoggedIn(): Promise<boolean> {
    let sessionId = getCookie('sessionid');
    if(sessionId == "") {
        return false;
    }

    let isLoggedInReq = $.ajax({
        url: SESSION_ENDPOINT,
        method: 'POST',
        data: {
            session_id: sessionId
        }
    });

    isLoggedInReq.fail((e) => {
        console.log(e);
    });

    let isLoggedInResponse = await isLoggedInReq;
    let response = <ISessionResponse> JSON.parse(isLoggedInResponse);

    return response.status == 200;
}

export async function doLogout() {
    
}