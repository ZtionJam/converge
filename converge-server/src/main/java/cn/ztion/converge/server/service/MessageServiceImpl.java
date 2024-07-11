package cn.ztion.converge.server.service;

import cn.ztion.converge.server.domain.Msg;
import cn.ztion.converge.server.holder.ClientHolder;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import jakarta.validation.constraints.NotEmpty;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import org.springframework.web.servlet.mvc.method.annotation.SseEmitter;

/**
 * MessageServiceImpl
 *
 * @author ZtionJam
 * @date 2024/6/25
 */
@Service
@Slf4j
public class MessageServiceImpl implements MessageService {
    private final ObjectMapper mapper = new ObjectMapper();

    @Override
    public void pushMsg(Msg msg) throws JsonProcessingException {
        log.info("receive msg:{}", msg);
        ClientHolder.push(msg.getId() + (msg.getId2() == null ? "" : msg.getId2()), msg);
    }

    @Override
    public SseEmitter listen(@NotEmpty String id) {
        SseEmitter emitter = new SseEmitter(10 * 24 * 60 * 60 * 1000L);
        ClientHolder.login(id, emitter);
        emitter.onCompletion(() -> ClientHolder.logOut(id, emitter));
        emitter.onError((e) -> ClientHolder.logOut(id, emitter));
        emitter.onTimeout(() -> ClientHolder.logOut(id, emitter));
        return emitter;
    }
}