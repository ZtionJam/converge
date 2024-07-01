package cn.ztion.converge.server.holder;

import cn.ztion.converge.server.domain.Msg;
import com.fasterxml.jackson.databind.ObjectMapper;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.MediaType;
import org.springframework.web.servlet.mvc.method.annotation.SseEmitter;

import java.util.ArrayList;
import java.util.Date;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

/**
 * ClientHolder
 *
 * @author ZtionJam
 * @date 2024/6/25
 */
@Slf4j
public class ClientHolder {

    private static final Map<String, List<SseEmitter>> CLIENTS = new ConcurrentHashMap<>();

    private static final ObjectMapper mapper = new ObjectMapper();


    public static int push(String id, Msg msg) {
        List<SseEmitter> sseEmitters = CLIENTS.get(id);
        int success = 0;
        if (sseEmitters != null) {
            msg.setDate(new Date());
            for (SseEmitter emitter : sseEmitters) {
                try {
                    msg.setTimes(success);
                    emitter.send(SseEmitter.event().data(mapper.writeValueAsString(msg), MediaType.TEXT_PLAIN));
                    success++;
                } catch (Throwable e) {
                    log.error("Message send error{}", id);
                }
            }
        }
        return success;
    }

    public static void push(String id, String msg) {
        List<SseEmitter> sseEmitters = CLIENTS.get(id);

        if (sseEmitters != null) {
            for (SseEmitter emitter : sseEmitters) {
                try {
                    emitter.send(SseEmitter.event().data(msg, MediaType.TEXT_PLAIN));
                } catch (Throwable e) {
                    log.error("Message send error{}", id);
                }
            }
        }
    }

    public static void login(String id, SseEmitter emitter) {
        List<SseEmitter> sseEmitters = CLIENTS.get(id);
        sseEmitters = new ArrayList<>(sseEmitters == null ? new ArrayList<>() : sseEmitters);
        sseEmitters.add(emitter);
        CLIENTS.put(id, sseEmitters);
        push(id, "ok");
    }

    public static void logOut(String id) {
        List<SseEmitter> sseEmitters = CLIENTS.get(id);
        if (sseEmitters != null) {
            CLIENTS.put(id, sseEmitters.stream().filter(ClientHolder::check).toList());
        }
    }

    private static boolean check(SseEmitter emitter) {
        try {
            emitter.send(SseEmitter.event().name("message").data("Are you Ok?"));
        } catch (Throwable e) {
            return false;
        }
        return true;
    }
}