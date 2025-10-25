//! Lexer modes and channels code generation.

use crate::ast::Grammar;

/// Generate lexer mode stack code for Rust.
pub fn generate_rust_mode_stack(grammar: &Grammar) -> String {
    if !grammar.has_modes() {
        return String::new();
    }

    let mut code = String::new();
    code.push_str("    /// Stack of active lexer modes\n");
    code.push_str("    mode_stack: Vec<String>,\n");

    if grammar.has_channels() {
        code.push_str("    /// Token channels\n");
        code.push_str("    channels: std::collections::HashMap<String, Vec<Token>>,\n");
    }

    code
}

/// Generate lexer mode methods for Rust.
pub fn generate_rust_mode_methods(grammar: &Grammar) -> String {
    if !grammar.has_modes() {
        return String::new();
    }

    let mut code = String::new();

    code.push_str("    /// Get current lexer mode\n");
    code.push_str("    #[inline]\n");
    code.push_str("    pub fn current_mode(&self) -> &str {\n");
    code.push_str("        self.mode_stack.last().unwrap_or(&\"DEFAULT_MODE\".to_string())\n");
    code.push_str("    }\n\n");

    code.push_str("    /// Switch to a different mode (pop current, push new)\n");
    code.push_str("    #[inline]\n");
    code.push_str("    pub fn switch_mode(&mut self, mode: &str) {\n");
    code.push_str("        if !self.mode_stack.is_empty() {\n");
    code.push_str("            self.mode_stack.pop();\n");
    code.push_str("        }\n");
    code.push_str("        self.mode_stack.push(mode.to_string());\n");
    code.push_str("    }\n\n");

    code.push_str("    /// Push a mode onto the stack\n");
    code.push_str("    #[inline]\n");
    code.push_str("    pub fn push_mode(&mut self, mode: &str) {\n");
    code.push_str("        self.mode_stack.push(mode.to_string());\n");
    code.push_str("    }\n\n");

    code.push_str("    /// Pop the current mode from the stack\n");
    code.push_str("    #[inline]\n");
    code.push_str("    pub fn pop_mode(&mut self) {\n");
    code.push_str("        if self.mode_stack.len() > 1 {\n");
    code.push_str("            self.mode_stack.pop();\n");
    code.push_str("        }\n");
    code.push_str("    }\n\n");

    if grammar.has_channels() {
        code.push_str("    /// Send token to a channel\n");
        code.push_str("    #[inline]\n");
        code.push_str("    pub fn send_to_channel(&mut self, channel: &str, token: Token) {\n");
        code.push_str("        self.channels.entry(channel.to_string())\n");
        code.push_str("            .or_insert_with(Vec::new)\n");
        code.push_str("            .push(token);\n");
        code.push_str("    }\n\n");

        code.push_str("    /// Get tokens from a channel\n");
        code.push_str("    #[inline]\n");
        code.push_str("    pub fn get_channel(&self, channel: &str) -> Vec<Token> {\n");
        code.push_str("        self.channels.get(channel).cloned().unwrap_or_default()\n");
        code.push_str("    }\n\n");
    }

    code
}

/// Generate lexer mode initialization for Rust.
pub fn generate_rust_mode_init(grammar: &Grammar) -> String {
    if !grammar.has_modes() {
        return String::new();
    }

    let mut code = String::new();
    code.push_str("        mode_stack: vec![\"DEFAULT_MODE\".to_string()],\n");

    if grammar.has_channels() {
        code.push_str("        channels: std::collections::HashMap::new(),\n");
    }

    code
}

/// Generate lexer mode stack code for Python.
pub fn generate_python_mode_stack(grammar: &Grammar) -> String {
    if !grammar.has_modes() {
        return String::new();
    }

    let mut code = String::new();
    code.push_str("        self.mode_stack = [\"DEFAULT_MODE\"]\n");

    if grammar.has_channels() {
        code.push_str("        self.channels = {}\n");
    }

    code
}

/// Generate lexer mode methods for Python.
pub fn generate_python_mode_methods(grammar: &Grammar) -> String {
    if !grammar.has_modes() {
        return String::new();
    }

    let mut code = String::new();

    code.push_str("    def current_mode(self) -> str:\n");
    code.push_str("        \"\"\"Get current lexer mode.\"\"\"\n");
    code.push_str("        return self.mode_stack[-1] if self.mode_stack else \"DEFAULT_MODE\"\n\n");

    code.push_str("    def switch_mode(self, mode: str) -> None:\n");
    code.push_str("        \"\"\"Switch to a different mode.\"\"\"\n");
    code.push_str("        if self.mode_stack:\n");
    code.push_str("            self.mode_stack.pop()\n");
    code.push_str("        self.mode_stack.append(mode)\n\n");

    code.push_str("    def push_mode(self, mode: str) -> None:\n");
    code.push_str("        \"\"\"Push a mode onto the stack.\"\"\"\n");
    code.push_str("        self.mode_stack.append(mode)\n\n");

    code.push_str("    def pop_mode(self) -> None:\n");
    code.push_str("        \"\"\"Pop the current mode from the stack.\"\"\"\n");
    code.push_str("        if len(self.mode_stack) > 1:\n");
    code.push_str("            self.mode_stack.pop()\n\n");

    if grammar.has_channels() {
        code.push_str("    def send_to_channel(self, channel: str, token: Token) -> None:\n");
        code.push_str("        \"\"\"Send token to a channel.\"\"\"\n");
        code.push_str("        if channel not in self.channels:\n");
        code.push_str("            self.channels[channel] = []\n");
        code.push_str("        self.channels[channel].append(token)\n\n");

        code.push_str("    def get_channel(self, channel: str) -> list[Token]:\n");
        code.push_str("        \"\"\"Get tokens from a channel.\"\"\"\n");
        code.push_str("        return self.channels.get(channel, [])\n\n");
    }

    code
}

/// Generate lexer mode stack code for JavaScript.
pub fn generate_javascript_mode_stack(grammar: &Grammar) -> String {
    if !grammar.has_modes() {
        return String::new();
    }

    let mut code = String::new();
    code.push_str("        this.modeStack = [\"DEFAULT_MODE\"];\n");

    if grammar.has_channels() {
        code.push_str("        this.channels = {};\n");
    }

    code
}

/// Generate lexer mode methods for JavaScript.
pub fn generate_javascript_mode_methods(grammar: &Grammar) -> String {
    if !grammar.has_modes() {
        return String::new();
    }

    let mut code = String::new();

    code.push_str("    currentMode() {\n");
    code.push_str("        return this.modeStack[this.modeStack.length - 1] || \"DEFAULT_MODE\";\n");
    code.push_str("    }\n\n");

    code.push_str("    switchMode(mode) {\n");
    code.push_str("        if (this.modeStack.length > 0) {\n");
    code.push_str("            this.modeStack.pop();\n");
    code.push_str("        }\n");
    code.push_str("        this.modeStack.push(mode);\n");
    code.push_str("    }\n\n");

    code.push_str("    pushMode(mode) {\n");
    code.push_str("        this.modeStack.push(mode);\n");
    code.push_str("    }\n\n");

    code.push_str("    popMode() {\n");
    code.push_str("        if (this.modeStack.length > 1) {\n");
    code.push_str("            this.modeStack.pop();\n");
    code.push_str("        }\n");
    code.push_str("    }\n\n");

    if grammar.has_channels() {
        code.push_str("    sendToChannel(channel, token) {\n");
        code.push_str("        if (!this.channels[channel]) {\n");
        code.push_str("            this.channels[channel] = [];\n");
        code.push_str("        }\n");
        code.push_str("        this.channels[channel].push(token);\n");
        code.push_str("    }\n\n");

        code.push_str("    getChannel(channel) {\n");
        code.push_str("        return this.channels[channel] || [];\n");
        code.push_str("    }\n\n");
    }

    code
}

/// Generate lexer mode stack code for TypeScript.
pub fn generate_typescript_mode_stack(grammar: &Grammar) -> String {
    if !grammar.has_modes() {
        return String::new();
    }

    let mut code = String::new();
    code.push_str("        this.modeStack: string[] = [\"DEFAULT_MODE\"];\n");

    if grammar.has_channels() {
        code.push_str("        this.channels: Map<string, Token[]> = new Map();\n");
    }

    code
}

/// Generate lexer mode methods for TypeScript.
pub fn generate_typescript_mode_methods(grammar: &Grammar) -> String {
    if !grammar.has_modes() {
        return String::new();
    }

    let mut code = String::new();

    code.push_str("    currentMode(): string {\n");
    code.push_str("        return this.modeStack[this.modeStack.length - 1] || \"DEFAULT_MODE\";\n");
    code.push_str("    }\n\n");

    code.push_str("    switchMode(mode: string): void {\n");
    code.push_str("        if (this.modeStack.length > 0) {\n");
    code.push_str("            this.modeStack.pop();\n");
    code.push_str("        }\n");
    code.push_str("        this.modeStack.push(mode);\n");
    code.push_str("    }\n\n");

    code.push_str("    pushMode(mode: string): void {\n");
    code.push_str("        this.modeStack.push(mode);\n");
    code.push_str("    }\n\n");

    code.push_str("    popMode(): void {\n");
    code.push_str("        if (this.modeStack.length > 1) {\n");
    code.push_str("            this.modeStack.pop();\n");
    code.push_str("        }\n");
    code.push_str("    }\n\n");

    if grammar.has_channels() {
        code.push_str("    sendToChannel(channel: string, token: Token): void {\n");
        code.push_str("        if (!this.channels.has(channel)) {\n");
        code.push_str("            this.channels.set(channel, []);\n");
        code.push_str("        }\n");
        code.push_str("        this.channels.get(channel)!.push(token);\n");
        code.push_str("    }\n\n");

        code.push_str("    getChannel(channel: string): Token[] {\n");
        code.push_str("        return this.channels.get(channel) || [];\n");
        code.push_str("    }\n\n");
    }

    code
}

/// Generate lexer mode stack code for Go.
pub fn generate_go_mode_stack(grammar: &Grammar) -> String {
    if !grammar.has_modes() {
        return String::new();
    }

    let mut code = String::new();
    code.push_str("    ModeStack []string\n");

    if grammar.has_channels() {
        code.push_str("    Channels map[string][]Token\n");
    }

    code
}

/// Generate lexer mode methods for Go.
pub fn generate_go_mode_methods(grammar: &Grammar) -> String {
    if !grammar.has_modes() {
        return String::new();
    }

    let mut code = String::new();

    code.push_str("// CurrentMode returns the current lexer mode\n");
    code.push_str("func (l *Lexer) CurrentMode() string {\n");
    code.push_str("    if len(l.ModeStack) == 0 {\n");
    code.push_str("        return \"DEFAULT_MODE\"\n");
    code.push_str("    }\n");
    code.push_str("    return l.ModeStack[len(l.ModeStack)-1]\n");
    code.push_str("}\n\n");

    code.push_str("// SwitchMode switches to a different mode\n");
    code.push_str("func (l *Lexer) SwitchMode(mode string) {\n");
    code.push_str("    if len(l.ModeStack) > 0 {\n");
    code.push_str("        l.ModeStack = l.ModeStack[:len(l.ModeStack)-1]\n");
    code.push_str("    }\n");
    code.push_str("    l.ModeStack = append(l.ModeStack, mode)\n");
    code.push_str("}\n\n");

    code.push_str("// PushMode pushes a mode onto the stack\n");
    code.push_str("func (l *Lexer) PushMode(mode string) {\n");
    code.push_str("    l.ModeStack = append(l.ModeStack, mode)\n");
    code.push_str("}\n\n");

    code.push_str("// PopMode pops the current mode from the stack\n");
    code.push_str("func (l *Lexer) PopMode() {\n");
    code.push_str("    if len(l.ModeStack) > 1 {\n");
    code.push_str("        l.ModeStack = l.ModeStack[:len(l.ModeStack)-1]\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    if grammar.has_channels() {
        code.push_str("// SendToChannel sends a token to a channel\n");
        code.push_str("func (l *Lexer) SendToChannel(channel string, token Token) {\n");
        code.push_str("    if l.Channels == nil {\n");
        code.push_str("        l.Channels = make(map[string][]Token)\n");
        code.push_str("    }\n");
        code.push_str("    l.Channels[channel] = append(l.Channels[channel], token)\n");
        code.push_str("}\n\n");

        code.push_str("// GetChannel returns tokens from a channel\n");
        code.push_str("func (l *Lexer) GetChannel(channel string) []Token {\n");
        code.push_str("    return l.Channels[channel]\n");
        code.push_str("}\n\n");
    }

    code
}
