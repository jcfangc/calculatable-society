import { LocalStorageKeys } from "@/utils/browser/local-storage/localStorageKeys.util";

export const LocalStorageUtil = {
	set<T>(key: LocalStorageKeys, value: T): void {
		try {
			localStorage.setItem(key, JSON.stringify(value));
		} catch (error) {
			console.error(`Error setting localStorage key "${key}":`, error);
		}
	},
	get<T>(key: LocalStorageKeys): T | null {
		try {
			const item = localStorage.getItem(key);
			return item ? (JSON.parse(item) as T) : null;
		} catch (error) {
			console.error(`Error getting localStorage key "${key}":`, error);
			return null;
		}
	},
	remove(key: LocalStorageKeys): void {
		try {
			localStorage.removeItem(key);
		} catch (error) {
			console.error(`Error removing localStorage key "${key}":`, error);
		}
	},
};
