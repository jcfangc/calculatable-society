import { en } from "./langs/en.util";
import { zh } from "./langs/zh.util";

export const enum LanguageEnum {
	EN = "EN",
	ZH = "ZH",
}

export const languages = {
	[LanguageEnum.EN]: en,
	[LanguageEnum.ZH]: zh,
};
