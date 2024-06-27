<template>
    <div :style="background" class="container">
        {{ msg }}
    </div>
</template>
<script setup>
import { ref } from 'vue';


let props = defineProps({
    msg: {
        type: String,
        default: 'This is a test msg'
    }
})
let background = ref();

if (props.msg.indexOf("✔") >= 0) {
    background.value = { background: 'rgb(88, 230, 119)' };
}

</script>
<style scoped lang="scss">
.container {
    padding-left: 10px;
    padding-right: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 30px;
    border-radius: 5px;
    background: orange;
    position: fixed;
    top: 40px;
    left: 50%;
    transform: translate(-50%, -50%);
    opacity: 0;
    animation: slideDown 200ms ease forwards;
    font-size: 14px;
    box-shadow: 0 0px 5px rgba(00, 00, 00, 0.25);
}

@keyframes slideDown {
    from {
        top: 40px;
        opacity: 0;
    }

    to {
        top: 60px;
        opacity: 1;
    }
}
</style>