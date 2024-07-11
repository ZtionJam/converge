<template>
    <div class="container">
        <div class="config_line">
            <p>服务器地址:</p><input type="text" placeholder="消息中转服务器地址，必填" v-model="data.host">
        </div>
        <div class="config_line">
            <p>ID:</p><input type="text" placeholder="唯一标识，必填" v-model="data.id">
        </div>
        <div class="config_line">
            <p>ID2:</p><input type="text" placeholder="可选的第二个唯一标识" v-model="data.id2">
        </div>
        <div class="config_line">
            <p>消息通知:</p>
            <div class="radio">
                <input type="radio" :value="true" name="notify" title="开启后，收到新消息将推送系统消息"
                    v-model="data.notify"><span>开启</span>
                <input checked :value="false" type="radio" name="notify" v-model="data.notify"><span>关闭</span>
            </div>
        </div>
        <div class="btns">
            <div class="save" @click="save">保存并连接</div>
        </div>
    </div>
</template>
<script setup>
import { onMounted, ref } from 'vue';
import Notice from '@/components/js/notice.js';
import { invoke } from '@tauri-apps/api/tauri'
import { tauri } from '@tauri-apps/api';
import { emit, listen } from '@tauri-apps/api/event'

let data = ref({
    host: "http://msgs.ztion.cn/msg/listen",
    id: "demo",
    id2: "",
    notify: false
})

onMounted(() => {
    let server = localStorage.getItem("server");
    if (server.length > 0) {
        data.value = JSON.parse(server);
    }
})

const save = () => {
    if (data.value.host.length == 0 || data.value.id.length == 0) {
        Notice("❌服务器地址和ID是必填的");
        return;
    }
    localStorage.setItem("server", JSON.stringify(data.value));
    Notice("✔保存成功")

    invoke("connect", { server: { ...data.value } })
}
listen("notify", e => {
  Notice(e.payload.message);
});
</script>
<style scoped lang="scss">
.container {
    padding: 10px;

    .config_line {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
        height: 50px;
        transition: all 200ms;
        background: #f1f1f1;
        border-radius: 5px;
        margin-top: 5px;

        &:hover {
            transition: all 200ms;
            transform: scale(1.01);
        }


        >p {
            display: inline-block;
            width: 100px;
            font-size: 16px;
            font-weight: bold;
            user-select: none;
        }

        >input[type='text'] {
            width: 250px;
            height: 65%;
            border: 1px solid #ccc;
            border-radius: 5px;
            padding-left: 5px;
            padding-right: 5px;
            text-overflow: ellipsis;
            white-space: nowrap;
            text-align: center;
            box-sizing: border-box;

            &:focus {
                border: 2px solid orange;
                outline: none;
            }
        }

        .radio {
            width: 250px;
            display: flex;
            justify-content: center;
            align-items: center;

            >input {
                margin-left: 10px;

                &:checked {
                    border: 2px solid orange;
                    outline: none;
                }
            }

            >span {
                margin-left: 5px;
                font-size: 14px;
            }

        }
    }

    .btns {
        width: 100%;
        height: 50px;
        margin-top: 10px;
        display: flex;
        align-items: center;
        justify-content: center;

        .save {
            width: 120px;
            background-color: orange;
            height: 35px;
            border-radius: 5px;
            text-align: center;
            line-height: 35px;
            font-weight: bold;
            color: white;
            box-shadow: 0 0 5px rgba(23, 82, 30, 0.25);
            user-select: none;

            &:hover {
                transition: all 200ms;
                cursor: pointer;
                box-shadow: 0 0 5px rgba(23, 82, 30, 0.5);
            }

            &:active {
                transition: all 200ms;
                transform: scale(1.1);
                cursor: pointer;
            }
        }
    }
}
</style>