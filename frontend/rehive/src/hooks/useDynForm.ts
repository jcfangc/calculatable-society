import { createVNode, render } from "vue";
import DynForm, { FormField } from "@/components/DynForm.vue";

export function useDynForm(
	fields: FormField[],
	onSubmit: (formData: Record<string, string>) => void,
	onCancel: () => void
): void {
	const container = document.createElement("div");
	document.body.appendChild(container);

	const handleClose = () => {
		render(null, container);
		document.body.removeChild(container);
	};

	const handleCancel = () => {
		onCancel();
		handleClose();
	};

	const vnode = createVNode(DynForm, {
		fields,
		onSubmit: (formData: Record<string, string>) => {
			onSubmit(formData);
			handleClose();
		},
		onCancel: handleCancel,
	});
	render(vnode, container);
}
