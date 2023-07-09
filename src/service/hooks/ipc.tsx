import { invokeCommand } from '@Service/core';
import { Mutation, Query } from '@Service/types';
import { IpcResponse } from '@Service/types/common';
import { useState, useEffect, useCallback } from 'react';

export interface UseInvokerOptions {
  autoInvoke: boolean;
}

export type UseInvokerResponse<I, R> = [
  { response: R | null; error: string | null; loading: boolean },
  (input?: I) => Promise<void>
];

const initValue = {
  result: null,
  error: null,
};

const defaultOptions: UseInvokerOptions = {
  autoInvoke: true,
};

export function useInvoker<I, R>(
  command: Query | Mutation,
  params?: I | any,
  opts?: UseInvokerOptions
): UseInvokerResponse<I, R> {
  const [response, setResponse] = useState<IpcResponse<R>>(initValue);
  const [loading, setLoading] = useState(false);
  const options = { ...defaultOptions, ...opts };

  const invokeFn = useCallback(async (invokeInput?: I) => {
    setLoading(true);
    const rawInput = (invokeInput ?? params) as any;
    const input = rawInput?.order
      ? { ...rawInput, order: rawInput.order?.toString() }
      : rawInput;
    const res = await invokeCommand<R>(command, input as any);
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
