package cn.ztion.converge.server.service;

import cn.ztion.converge.server.domain.Msg;
import cn.ztion.converge.server.holder.ClientHolder;
import jakarta.validation.constraints.NotEmpty;
import org.springframework.stereotype.Service;
import org.springframework.web.servlet.mvc.method.annotation.SseEmitter;

/**
 * MessageServiceImpl
 *
 * @author ZtionJam
 * @date 2024/6/25
 */
@Service
public class MessageServiceImpl implements MessageService {


    @Override
    public void pushMsg(Msg msg) {
        int push = ClientHolder.push(msg.getId() + msg.getId2(), msg.getContent());
        msg.setTimes(push);
    }

    @Override
    public SseEmitter listen(@NotEmpty String id) {
        SseEmitter emitter = new SseEmitter(10 * 60 * 1000L);
        ClientHolder.login(id, emitter);
        emitter.onCompletion(() -> ClientHolder.logOut(id));
        emitter.onError((e) -> ClientHolder.logOut(id));
        emitter.onTimeout(() -> ClientHolder.logOut(id));
        return emitter;
    }
}