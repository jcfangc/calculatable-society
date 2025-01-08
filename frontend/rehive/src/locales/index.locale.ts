import { en } from "./langs/en.locale";
import { zh } from "./langs/zh.locale";

export const enum LanguageEnum {
	EN = "EN",
	ZH = "ZH",
}

export const languages = {
	[LanguageEnum.EN]: en,
	[LanguageEnum.ZH]: zh,
};
