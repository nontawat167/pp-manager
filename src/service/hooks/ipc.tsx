import { invokeCommand } from '@Service/core';
import { useState, useEffect, useCallback } from 'react';

export interface UseInvokerOptions {
  autoInvoke: boolean;
}

type InvokeFn = () => Promise<void>;
export type UseInvokerResponse<T> = [
  { response: T | null; error: string | null; loading: boolean },
  InvokeFn
];

const initValue = {
  result: null,
  error: null,
};

const defaultOptions: UseInvokerOptions = {
  autoInvoke: true,
};

export function useInvoker<T>(
  command: string,
  params?: any,
  opts?: UseInvokerOptions
): UseInvokerResponse<T> {
  const [response, setResponse] = useState<IpcResponse<T>>(initValue);
  const [loading, setLoading] = useState(false);
  const options = { ...defaultOptions, ...opts };

  const invokeFn = useCallback(async () => {
    setLoading(true);
    const res = await invokeCommand<T>(command, params);
    setResponse(res);
    setLoading(false);
  }, []);

  useEffect(() => {
    if (options.autoInvoke) invokeFn();
  }, []);

  return [
    {
      response: response.result === null ? null : response.result.data,
      error: response.error,
      loading,
    },
    invokeFn,
  ];
}
