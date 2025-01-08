import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import pluginVue from "eslint-plugin-vue";

/** @type {import('eslint').Linter.Config[]} */
export default [
  {files: ["**/*.{ts,vue}"]}, // 检查 TypeScript 和 Vue 文件
  {languageOptions: { globals: globals.browser }}, // 设置全局变量
  pluginJs.configs.recommended, // JavaScript 推荐配置
  ...tseslint.configs.recommended, // TypeScript 推荐配置
  ...pluginVue.configs["flat/essential"], // Vue 必要规则配置
  {
    files: ["**/*.vue"], // 仅针对 Vue 文件的配置
    languageOptions: {
      parserOptions: { parser: tseslint.parser } // 设置 TypeScript 解析器
    },
    rules: {
      "vue/multi-word-component-names": "off", // 允许单词组件名
      "complexity": ["error", { "max": 10 }] // 圈复杂度检查，最大值为 10
    }
  },
  {
    files: ["**/*.{ts}"], // 针对 TypeScript 和 JavaScript 文件
    rules: {
      "complexity": ["error", { "max": 10 }] // 限制最大圈复杂度为 10
    }
  }
];
