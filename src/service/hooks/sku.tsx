import { Sku } from '@Service/types/sku';
import { UseInvokerOptions, UseInvokerResponse, useInvoker } from './ipc';

export const useAllSKU = (
  opts?: UseInvokerOptions
): UseInvokerResponse<Sku[]> => {
  const result = useInvoker<Sku[]>('get_skus', undefined, opts);
  return result;
};
