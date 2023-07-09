import { OrderFilter } from '@Utils/Order';

interface Sku {
  id: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
  name: string;
  price: number;
  product_type: string;
}

interface CreateSkuInput {
  name: string;
  price: number;
  product_type: string;
}

interface ISearchSkusInput {
  name: string;
  price: number;
  product_type: string;

  page: number;
  per_page: number;
  order: OrderFilter;
}

interface TauriSearchSkusInput {
  name: string;
  price: number;
  product_type: string;

  page: number;
  per_page: number;
  order: string;
}

type SearchSkusInput = Partial<ISearchSkusInput>;

interface SkuSearchResponse {
  total: number;
  skus: Sku[];
}

export type {
  Sku,
  CreateSkuInput,
  SearchSkusInput,
  TauriSearchSkusInput,
  SkuSearchResponse,
};
