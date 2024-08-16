# Simx So Extension Document

```json
{
  "name": "插件名称",
  "version": "插件版本",
  "engine": "兼容的引擎版本范围",
  "description": "插件描述",
  "license": "开源许可，非开源设置为none",
  "author": "插件作者",
  "keywords": [
    "关键字"
  ],
  "dependencies": [
    "依赖的其他插件"
  ],
  "entry_lib": "入口库所在路径",
  "entry_func": "入口方法名称"
}
```

## 强制规定

### 默认方法：

#### extension.init

引擎会调用此方法来执行插件的初始化操作

#### extension.destroy

引擎会调用此方法来执行插件的销毁操作

#### extension.info

设计器会调用此方法，获取插件中的方法信息，需要返回的对象格式如下：

```json
[
  {
    "name": "方法名称",
    "description": "方法描述",
    "params": [
      "参数类型 参数名称"
    ],
    "return": "返回值类型",
    "example": "强制同步"
  }
]
```