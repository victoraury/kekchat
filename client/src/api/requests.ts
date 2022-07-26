async function fetcharoo(url: string, method: string, body: any = undefined) {
    return await fetch(url, {
        method,
        body,
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        }
    });
}

export async function get(url: string, body: any = undefined): Promise<{ok: true, json: any} | {ok: false, error: string}> {

    let r = await fetcharoo(url, "GET", body);
    let json = await r.json();

    if( r.ok ) {
        return { ok: true, json }
    }
    else {
        return { ok: false, error: json.error }
    }
}

export async function post(url: string, body: any = undefined): Promise<{ok: true, json: any} | {ok: false, error: string}> {

    let r = await fetcharoo(url, "POST", body);
    let json = await r.json();

    if( r.ok ) {
        return { ok: true, json }
    }
    else {
        return { ok: false, error: json.error }
    }
}
