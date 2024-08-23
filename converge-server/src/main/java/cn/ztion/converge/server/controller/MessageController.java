package cn.ztion.converge.server.controller;

import cn.ztion.converge.server.domain.Msg;
import cn.ztion.converge.server.domain.R;
import cn.ztion.converge.server.service.MessageService;
import lombok.AllArgsConstructor;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.servlet.mvc.method.annotation.SseEmitter;

/**
 * MessageController
 *
 * @author ZtionJam
 * @date 2024/6/25
 */
@RestController
@RequestMapping("/msg")
@AllArgsConstructor
public class MessageController {

    private final MessageService messageService;

    /**
     * 消息接受并推送到已连接的客户端
     *
     * @param msg 消息
     */
    @PostMapping
    public R<Msg> upload(@RequestBody @Validated Msg msg) throws Exception {
        this.messageService.pushMsg(msg);
        return R.ok(msg);
    }

    /**
     * SSE链接
     */
    @GetMapping("/listen")
    public SseEmitter listen(@RequestParam String id, String id2) {
        return this.messageService.listen(id + (id2 == null ? "" : id2));
    }
}