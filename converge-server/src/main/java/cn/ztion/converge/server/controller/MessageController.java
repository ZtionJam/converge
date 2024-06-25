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

    @PostMapping
    public R<Msg> upload(@RequestBody @Validated Msg msg) {
        this.messageService.pushMsg(msg);
        return R.ok(msg);
    }

    @GetMapping("/listen")
    public SseEmitter listen(@RequestParam String id, String id2) {
        return this.messageService.listen(id + id2);
    }
}