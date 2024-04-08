import { CreateSkuInput, Sku, SkuSearchResponse } from '@Service/types/sku';
import { Query, Mutation } from '@Service/types';
import { useMutation, useQuery } from '@tanstack/react-query';
import { invokeCommand } from '@Service/core';

export const useSearchSkus = () => {
  return useQuery({
    queryKey: [Query.SEARCH_SKUS_2],
    queryFn: async () => {
      const result = await invokeCommand<SkuSearchResponse>(
        Query.SEARCH_SKUS_2
      );
      return result.result?.data;
    },
  });
};

export const useCreateSku = () => {
  return useMutation({
    mutationFn: async (createSkuInput: Partial<CreateSkuInput>) => {
      const result = await invokeCommand<Sku>(
        Mutation.CREATE_SKU,
        createSkuInput
      );
      return result.result?.data;
    },
  });
};
