import { InvokeArgs, invoke } from '@tauri-apps/api/tauri';
import { IpcResponse } from './types/common';

export async function invokeCommand<T>(
  command: string,
  params?: InvokeArgs
): Promise<IpcResponse<T>> {
  const result = (await invoke(command, { input: params })) as IpcResponse<T>;
  return result;
}
