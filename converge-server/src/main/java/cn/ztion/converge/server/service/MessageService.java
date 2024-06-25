package cn.ztion.converge.server.service;

import cn.ztion.converge.server.domain.Msg;
import org.springframework.web.servlet.mvc.method.annotation.SseEmitter;


/**
 * MessageService
 *
 * @author ZtionJam
 * @date 2024/6/25
 */
public interface MessageService {
    /**
     * Push msg
     *
     * @param msg msg
     */
    void pushMsg(Msg msg);

    /**
     * Listen id
     *
     * @param id id
     */
    SseEmitter listen(String id);
}
