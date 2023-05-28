import { InvokeArgs, invoke } from '@tauri-apps/api/tauri';

export async function invokeCommand<T>(
  command: string,
  params?: InvokeArgs
): Promise<IpcResponse<T>> {
  const result = (await invoke(command, params)) as IpcResponse<T>;
  return result;
}
