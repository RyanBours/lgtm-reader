import { error } from '@sveltejs/kit';
import { invoke } from '@tauri-apps/api/tauri';

/** @type {import('./$types').PageLoad} */
export function load({ params }) {
    const fetchContent = async () => {
        const res = await invoke('cbz_info', {
            // TEMP: hardcoded path
            fileLocation: 'F:/projects/lgtm-reader/src-tauri/library/' + decodeURI(params.slug)
        });
        return res;
    };

    const fetchThumbnail = async () => {
        const res = await invoke('cbz_thumbnail', {
            fileLocation: 'F:/projects/lgtm-reader/src-tauri/library/' + decodeURI(params.slug)
        });
        return 'data:image/png;base64,' + res;
    };

    const fetchPages = async () => {
        const res = await invoke('cbz_page_images', {
            fileLocation: 'F:/projects/lgtm-reader/src-tauri/library/' + decodeURI(params.slug)
        });
        return res;
    }

    return { info: fetchContent(), thumbnail: fetchThumbnail(), pages: fetchPages() };

    // throw error(404, 'Not found');
}