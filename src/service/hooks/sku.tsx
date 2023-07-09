import {
  CreateSkuInput,
  SearchSkusInput,
  Sku,
  SkuSearchResponse,
} from '@Service/types/sku';
import { Query, Mutation } from '@Service/types';
import { UseInvokerOptions, UseInvokerResponse, useInvoker } from './ipc';

export const useSearchSkus = (
  searchInput: SearchSkusInput,
  opts?: UseInvokerOptions
): UseInvokerResponse<SearchSkusInput, SkuSearchResponse> => {
  const result = useInvoker<SearchSkusInput, SkuSearchResponse>(
    Query.SEARCH_SKUS,
    searchInput,
    opts
  );

  return result;
};

export const useCreateSku = (
  createSkuInput: Partial<CreateSkuInput>,
  opts?: UseInvokerOptions
): UseInvokerResponse<CreateSkuInput, Sku> => {
  const result = useInvoker<CreateSkuInput, Sku>(
    Mutation.CREATE_SKU,
    createSkuInput,
    opts
  );

  return result;
};
