import { error } from '@sveltejs/kit';
import { invoke } from '@tauri-apps/api/tauri';

/** @type {import('./$types').PageLoad} */
export function load({ params }) {
    const fetchImage = async () => {
        const res = await invoke('cbz_page_name', {
            // TEMP: hardcoded path
            fileLocation: 'F:/projects/lgtm-reader/src-tauri/' + decodeURI(location.pathname).slice(0, -(params.slug.length + 1)),
            page: params.slug
        });
        return res;
    };

    const fetchPages = async () => {
        const path = 'F:/projects/lgtm-reader/src-tauri/' + decodeURI(location.pathname)
        console.log(path);
        const res = await invoke('cbz_pages', {
            fileLocation: 'F:/projects/lgtm-reader/src-tauri/' + decodeURI(location.pathname).slice(0, -(params.slug.length + 1)),
        });
        console.log(res);
        return res;
    }

    return {
        img: fetchImage(), pages: fetchPages(), slug: params.slug
    };

    // throw error(404, 'Not found');
}