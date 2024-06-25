package cn.ztion.converge.server.domain;

import lombok.Data;
import lombok.experimental.Accessors;

/**
 * Resullt
 *
 * @author ZtionJam
 * @date 2024/6/25
 */
@Data
@Accessors(chain = true)
public class R<T> {


    private int code;
    private String msg;
    private T data;


    public static <U> R<U> ok(U data) {
        return new R<U>()
                .setCode(200)
                .setMsg("ok")
                .setData(data);
    }
}