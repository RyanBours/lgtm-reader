import { error } from '@sveltejs/kit';
import { invoke } from '@tauri-apps/api/tauri';

/** @type {import('./$types').PageLoad} */
export function load({ params }) {
    const fetchContent = async () => {
        const res = await invoke('cbz_info', {
            // TEMP: hardcoded path
            fileLocation: 'F:/projects/lgtm-reader/src-tauri/library/' + decodeURI(params.slug)
        });
        console.log(res);
        return res;
    };

    return fetchContent();

    // throw error(404, 'Not found');
}