type IpcResponse<T> = {
  error: string | null;
  result: { data: T } | null;
};
