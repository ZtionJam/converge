<template>
  <div class="container">
    <div class="empty" v-if="data.msgList.length == 0">
      <div><img src="@/assets/icon/empty.png" /></div>
      <div>还没有消息哦</div>
    </div>
    <div class="msg_list">
      <div class="msg_card" v-for="msg in data.msgList" :key="msg.content">
        <div class="msg_sender">
          <div>{{ msg.sender }}</div>
          <div title="这条信息推送的客户端数量">{{ msg.times }}</div>
        </div>
        <div class="msg_content">{{ msg.content }}</div>
        <div class="msg_date">{{ msg.date }}</div>
      </div>
    </div>

    <div class="fixed_btn">
      <div title="设置" @click="open_setting">
        <img src="@/assets/icon/setting.png" alt />
        <div class="status_text" :style="{ 'color': data.connect ? 'green' : 'red' }">{{ data.connect ? '已连接' : '未连接' }}
        </div>
      </div>
    </div>
  </div>
</template>
<script setup>
import { ref, onMounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import Notice from "@/components/js/notice.js";

let data = ref({
  msgList: [
    // {
    //   content:
    //     "【知乎】你的验证码是 756332，此验证码用于登录知乎或重置密码。10 分钟内有效。",
    //   times: 2,
    //   sender: "1882821192",
    //   date: "2024-08-12 12:22:00"
    // }
  ],
  server: {
    host: "http://127.0.0.1:8081/msg/listen",
    id: "123",
    id2: "123"
  },
  connect: true
});
onMounted(() => {
  let server = localStorage.getItem("server");
  if (server.length > 0) {
    console.log(123)
    console.log(server);
    let s = JSON.parse(server);
    invoke("connect", { server: { ...s } });
  }
  invoke("get_history").then(msgs=>{
    data.value.msgList.push(...msgs);
  });
});
const open_setting = () => {
  invoke("setting");
};
listen("notify", e => {
  let msg = e.payload.message;
  if (msg.indexOf("✔") >= 0) {
    data.value.connect = true;
  } else if (msg.indexOf("❌") >= 0) {
    data.value.connect = false;
  }
  Notice(msg);
});
listen("msg", e => {
  console.log(e.payload);
  data.value.msgList.push(JSON.parse(e.payload.message));
});
</script>
<style scoped lang="scss">
.empty {
  width: 100%;
  height: 500px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  user-select: none;

  >div:nth-child(1) {
    opacity: 0.3;

  }

  >div:nth-child(2) {
    color: #aaa;
    font-weight: bold;
    margin-top: 10px;
  }
}

.msg_list {
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;

  .msg_card {
    width: 90%;
    min-height: 100px;
    border: 1px solid #ccc;
    margin-top: 15px;
    box-shadow: 0 0px 5px rgba(23, 82, 30, 0.25);
    border-radius: 5px;
    padding: 5px;

    .msg_sender {
      height: 30px;
      width: 100%;

      >div:nth-child(1) {
        float: left;
        text-indent: 10px;
      }

      >div:nth-child(2) {
        float: right;
        width: 25px;
        height: 25px;
        background-repeat: no-repeat;
        background-image: url("/src/assets/icon/rt.png");
        background-size: 100%;
        margin-top: -5px;
        margin-right: -5px;
        text-align: center;
        text-indent: 12px;
        font-size: 13px;
        font-weight: bold;
        color: white;
        user-select: none;
      }
    }

    .msg_content {
      text-indent: 2em;
    }

    .msg_date {
      margin-top: 10px;
      font-size: 12px;
      text-align: right;
      margin-right: 10px;
    }
  }
}

.fixed_btn {
  position: fixed;
  width: 50px;
  right: 20px;
  bottom: 50px;

  >div {
    margin-top: 10px;
    width: 100%;
    height: 50px;

    .status_text {
      color: green;
      text-align: center;
      font-weight: bold;
      cursor: pointer;

    }

    >img {
      width: 50px;
      filter: drop-shadow(0 0 3px rgba(0, 0, 0, 0.3));
      transition: all 200ms;

      &:hover {
        transition: all 200ms;
        transform: scale(1.1);
        filter: drop-shadow(0 0 5px rgba(0, 0, 0, 0.3));
        cursor: pointer;
      }

      &:active {
        transition: all 200ms;
        transform: scale(1.2);
        filter: drop-shadow(0 0 8px rgba(0, 0, 0, 0.3));
        cursor: pointer;
      }
    }
  }
}
</style>