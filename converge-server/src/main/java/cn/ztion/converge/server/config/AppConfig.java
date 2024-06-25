package cn.ztion.converge.server.config;

import cn.ztion.converge.server.domain.AppConfigProp;
import org.springframework.boot.context.properties.EnableConfigurationProperties;
import org.springframework.context.annotation.Configuration;

/**
 * AppConfig
 *
 * @author ZtionJam
 * @date 2024/6/25
 */
@Configuration
@EnableConfigurationProperties(AppConfigProp.class)
public class AppConfig {

}