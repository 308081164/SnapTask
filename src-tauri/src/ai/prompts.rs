use crate::ai::models::{ChatMessage, AnalysisContext};

/// 系统提示词
pub const SYSTEM_PROMPT: &str = r#"你是一个专业的任务管理助手，负责从截屏图片中提取任务信息。请遵循以下规则：

## 任务提取规则
1. 从截屏中识别出所有待办事项、任务、待处理的工作项
2. 每个任务需要提取：标题（必填）、描述（可选）、截止日期（可选）、优先级（可选）
3. 如果截屏中没有明确的任务，返回空的任务列表

## 时间提取规则
1. 识别截屏中的日期和时间信息
2. 将相对时间转换为绝对日期（基于当前日期：{current_date}）
3. 日期格式统一为 YYYY-MM-DD
4. 如果提到"明天"、"下周一"等，请计算具体日期
5. 如果提到具体时间如"下午3点"，请包含在截止日期中

## 优先级推断规则
1. urgent: 标记为紧急、重要且紧急、ASAP、立即处理
2. high: 标记为重要、高优先级、今天必须完成
3. medium: 标记为普通优先级、本周内完成
4. low: 标记为低优先级、有空再做、不急
5. none: 无法判断优先级

## 场景类型识别
- "work": 工作相关（邮件、文档、项目管理工具）
- "chat": 聊天记录（微信、钉钉、Slack 等）
- "browser": 浏览器页面
- "code": 代码/开发相关
- "other": 其他场景

## 活跃客户列表
{active_clients}

## 活跃项目列表
{active_projects}

## 输出格式
请严格按以下 JSON 格式输出，不要包含其他文字：
{
  "tasks": [
    {
      "title": "任务标题",
      "description": "任务详细描述（可选）",
      "client_name": "关联客户名称（可选，需从活跃客户列表匹配）",
      "deadline": "YYYY-MM-DD 或 YYYY-MM-DD HH:MM（可选）",
      "priority": "none|low|medium|high|urgent",
      "tags": ["标签1", "标签2"],
      "confidence": 0.0-1.0,
      "reasoning": "提取理由"
    }
  ],
  "ocr_text": "截屏中识别到的所有文字内容",
  "scene_type": "work|chat|browser|code|other"
}"#;

/// 用户提示词模板
pub const USER_PROMPT_TEMPLATE: &str = r#"请分析这张截屏，提取其中的任务信息。

当前时间：{current_date} {current_time}

请识别截屏中的所有待办事项和任务，并按 JSON 格式返回。"#;

/// 构建分析消息列表
pub fn build_analysis_messages(context: &AnalysisContext) -> Vec<ChatMessage> {
    let active_clients_str = if context.active_clients.is_empty() {
        "（暂无活跃客户）".to_string()
    } else {
        context.active_clients.join(", ")
    };

    let active_projects_str = if context.active_projects.is_empty() {
        "（暂无活跃项目）".to_string()
    } else {
        context.active_projects.join(", ")
    };

    let system_content = SYSTEM_PROMPT
        .replace("{current_date}", &context.current_date)
        .replace("{active_clients}", &active_clients_str)
        .replace("{active_projects}", &active_projects_str);

    let user_content = USER_PROMPT_TEMPLATE
        .replace("{current_date}", &context.current_date)
        .replace("{current_time}", &context.current_time);

    vec![
        ChatMessage {
            role: "system".to_string(),
            content: serde_json::Value::String(system_content),
        },
        ChatMessage {
            role: "user".to_string(),
            content: serde_json::Value::String(user_content),
        },
    ]
}

/// 构建带图片的消息
pub fn build_image_message(image_base64: &str) -> ChatMessage {
    ChatMessage {
        role: "user".to_string(),
        content: serde_json::json!([
            {
                "type": "image_url",
                "image_url": {
                    "url": format!("data:image/png;base64,{}", image_base64)
                }
            },
            {
                "type": "text",
                "text": "请分析上面的截屏图片。"
            }
        ]),
    }
}
