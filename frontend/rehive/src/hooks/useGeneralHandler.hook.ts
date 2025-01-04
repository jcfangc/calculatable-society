interface AsyncHandlerOptions<
  PreParams = Record<string, any>,
  PreprocessingReturnType = void,
  TaskParams = Record<string, any>,
  TaskReturnType = unknown,
  PostParams = Record<string, any>,
  PostReturnType = void
> {
  // 前置处理逻辑
  preProcessing?: (
    params?: PreParams,
    signal?: AbortSignal
  ) => Promise<PreprocessingReturnType>;

  // 主任务逻辑
  task: (
    preResult?: PreprocessingReturnType,
    params?: TaskParams,
    signal?: AbortSignal
  ) => Promise<TaskReturnType>;

  // 后置处理逻辑
  postProcessing?: (
    taskResult: TaskReturnType,
    params?: PostParams,
    signal?: AbortSignal
  ) => Promise<PostReturnType>;

  // 错误处理逻辑
  onError?: (error: Error) => void;

  // 超时时间（全局默认）
  timeout?: number;

  // 取消控制器（全局默认）
  controller?: AbortController;

  // 并发控制
  maxConcurrency?: number;

  // 任务队列：支持灵活配置任务
  taskQueue?: Array<{
    preProcessing?: (
      params?: PreParams,
      signal?: AbortSignal
    ) => Promise<PreprocessingReturnType>;
    task: (
      preResult?: PreprocessingReturnType,
      params?: TaskParams,
      signal?: AbortSignal
    ) => Promise<TaskReturnType>;
    postProcessing?: (
      taskResult: TaskReturnType,
      params?: PostParams,
      signal?: AbortSignal
    ) => Promise<PostReturnType>;
    onError?: (error: Error) => void;
    timeout?: number;
    controller?: AbortController;
  }>;
}
