package cn.ztion.converge.server.holder;

import lombok.extern.slf4j.Slf4j;
import org.springframework.http.MediaType;
import org.springframework.web.servlet.mvc.method.annotation.SseEmitter;

import java.util.ArrayList;
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


    public static int push(String id, String data) {
        List<SseEmitter> sseEmitters = CLIENTS.get(id);
        int success = 0;
        if (sseEmitters != null) {
            for (SseEmitter emitter : sseEmitters) {
                try {
                    emitter.send(SseEmitter.event().data(data, MediaType.TEXT_PLAIN));
                    success++;
                } catch (Exception e) {
                    log.error("Message send error", e);
                }
            }
        }
        return success;
    }

    public static void login(String id, SseEmitter emitter) {
        List<SseEmitter> sseEmitters = CLIENTS.get(id);
        sseEmitters = new ArrayList<>(sseEmitters);
        sseEmitters.add(emitter);
        CLIENTS.put(id, sseEmitters);
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
        } catch (Exception e) {
            return false;
        }
        return true;
    }
}