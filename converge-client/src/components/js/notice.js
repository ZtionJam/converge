import { createApp, ref } from 'vue';
import NoticeComp from '../notice.vue';

function Notice(msg) {
    const noticeInstance = createApp(NoticeComp, {
        msg
    });

    const mountNode = document.createElement('div');
    document.body.appendChild(mountNode);
    noticeInstance.mount(mountNode);

    setTimeout(() => {
        noticeInstance.unmount(mountNode);
        document.body.removeChild(mountNode);
    }, 1500);
}
export default Notice;