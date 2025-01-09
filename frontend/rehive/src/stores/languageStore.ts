import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { languages, LanguageEnum } from "@/utils/locales/index.util";
import { TextKeys } from "@/utils/locales/keys.util";
import { LocalStorageKeys } from "@/utils/browser/local-storage/localStorageKeys.util";
import { LocalStorageUtil } from "@/utils/browser/local-storage/localStorage.util";

// 插值工具函数
const formatString = (
	template: string,
	variables: Record<string, string | number>
) => {
	return template.replace(
		/{(.*?)}/g,
		(_, key) => (variables[key] as string) || ""
	);
};

// 定义 Pinia Store
export const useLanguageStore = defineStore("language", () => {
	// 当前语言状态
	const currentLanguage = ref<LanguageEnum>(
		LocalStorageUtil.get<LanguageEnum>(LocalStorageKeys.LANGUAGE) ||
			LanguageEnum.ZH
	);

	// 获取当前语言包
	const langPack = computed(() => languages[currentLanguage.value]);

	// 翻译函数
	const t = (
		key: TextKeys,
		vars?: Record<string, string | number>
	): string => {
		const template = langPack.value[key] || key; // 支持默认值
		return vars ? formatString(template, vars) : template;
	};

	// 切换语言
	const setLanguage = (lang: LanguageEnum): void => {
		if (languages[lang]) {
			currentLanguage.value = lang; // 更新语言
			LocalStorageUtil.set(LocalStorageKeys.LANGUAGE, lang); // 保存到本地
		} else {
			console.warn(`Unsupported language: ${lang}`);
		}
	};

	// 返回可用的方法和状态
	return {
		currentLanguage,
		langPack,
		t,
		setLanguage,
	};
});
