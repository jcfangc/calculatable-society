// 一键式创建场景，不需要关心内部逻辑
export { createThreeScene, makeThreeRefs } from "./createThreeScene";

// 拆分逻辑，如果遇到需要高度自定义的场景，使用下面拆分的 API 进行组合
export { createSceneAndCamera } from "./createSceneAndCamera";
export { createRenderer } from "./createRenderer";
export { createControls } from "./createControls";
export {
	createHexagonGeometry,
	createHexagonMatrixInstanced,
} from "./createHexagonInstancedMesh";
export { clearSceneObjects } from "./clearSceneObjects";
export { animate } from "./animate";
export { createCameraController } from "./createShortcuts";
