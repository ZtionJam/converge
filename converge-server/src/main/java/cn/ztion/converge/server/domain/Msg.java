package cn.ztion.converge.server.domain;

import com.fasterxml.jackson.annotation.JsonFormat;
import jakarta.validation.constraints.NotEmpty;
import lombok.Data;
import org.hibernate.validator.constraints.Length;

import java.util.Date;

/**
 * Message
 *
 * @author ZtionJam
 * @date 2024/6/25
 */
@Data
public class Msg {
    /**
     * id
     */
    @NotEmpty(message = "The first id can not be empty")
    @Length(max = 32, message = "Max length 32")
    private String id;
    /**
     * id2,Use two ids for extra security
     */
    @Length(max = 128, message = "Max length 32")
    private String id2;
    /**
     * sender
     */
    @Length(max = 32, message = "Max length 32")
    private String sender;
    /**
     * Message content
     */
    @NotEmpty(message = "The message content can not be empty")
    @Length(max = 512, message = "Max length 512")
    private String content;
    /**
     * The number of times this text message was pushed
     */
    private Integer times;
    /**
     * Rec Time
     */
    @JsonFormat(pattern = "yyyy-MM-dd HH:mm:ss")
    private Date date;


}