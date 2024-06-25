package cn.ztion.converge.server.domain;

import lombok.Data;
import org.springframework.boot.context.properties.ConfigurationProperties;

/**
 * AppConfigProp
 *
 * @author ZtionJam
 * @date 2024/6/25
 */
@ConfigurationProperties(prefix = "converge")
@Data
public class AppConfigProp {

    private Integer maxUser;

    private Integer maxUserConnect;

    private Integer maxTotalConnect;
}