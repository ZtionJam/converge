<template>
    <div class="container">
        <div class="head">
            <div class="title" data-tauri-drag-region>焦距</div>
            <div class="btns">
                <div @click="appWindow.minimize()" class="min">
                    <img src="@/assets/icon/min.png" alt />
                </div>
                <div @click="appWindow.close()" class="close">
                    <img src="@/assets/icon/close.png" alt />
                </div>
            </div>
        </div>
    </div>
</template>
<script setup>
import { appWindow } from "@tauri-apps/api/window";

</script>
<style scoped lang="scss">
.container {
    position: sticky;
    top: 0;
}

.head {
    height: 35px;
    width: 100%;
    background-color: #ccc;
    display: flex;

    .title {
        width: 80%;
        height: 100%;
        line-height: 35px;
        font-size: 14px;
        font-weight: bold;
        text-indent: 60%;
        user-select: none;
    }

    .btns {
        width: 20%;
        height: 100%;
        background: #888;
        display: flex;

        .close:hover {
            background: red;
        }

        >div {
            width: 50%;
            height: 100%;
            display: flex;
            justify-content: center;
            align-items: center;

            &:hover {
                cursor: pointer;
            }

            >img {
                width: 20px;
            }

        }
    }
}
</style>