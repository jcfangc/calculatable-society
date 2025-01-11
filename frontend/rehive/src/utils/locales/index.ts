import { en } from "./langs/en";
import { zh } from "./langs/zh";

export const enum LanguageEnum {
	EN = "EN",
	ZH = "ZH",
}

export const languages = {
	[LanguageEnum.EN]: en,
	[LanguageEnum.ZH]: zh,
};
