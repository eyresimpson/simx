# Simx Jar Extension Document

## 静态插件项目

> 静态插件项目，注意，这种方式只能调用Jar中的static方法

- [x] 调用静态方法
- [x] 无需启动Jar包
- [x] 调用速度较快

需要在bootstrap.yml中配置jar扩展，如果没有改文件，需要手动在src/resource/下创建一个，需要附加的内容如下：

```yaml
simx:
  # 项目名称
  name: simx-demo
  # 版本号
  version: 1.0.0
  # 描述信息
  description: 测试插件包
  # 作者
  author: simx
  # 作者邮箱
  email: demo@test.com
  # 授权方式，默认为none即可
  license: none
  # 扫描路径，系统会对指定的文件进行扫描
  function:
    scan:
      - src/main/java/com/zerocloud/custom/service/api/service/impl
    public:
      - com.zerocloud.custom.service.api.service.impl.CategoryServiceImpl.getCategoryList
      - com.zerocloud.custom.service.api.service.impl.MaterialServiceImpl.getMaterialList
```

**注意：不要试图将配置置放于Nacos中，此方法不会启动Jar包，因此无法获取到Nacos上的数据**

## 动态插件项目

**建议使用默认的插件项目进行开发**

> 动态插件项目，可以调用Jar中的所有方法

- [x] 调用所有方法
- [x] 需要启动Jar包
- [x] 需要等待Jar启动完成

