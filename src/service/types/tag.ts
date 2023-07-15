import { OrderFilter } from '@Utils/Order';

export enum TagKind {
  PRODUCT_TYPE = 'PRODUCT_TYPE',
}

interface Tag {
  id: string;
  name: string;
  kind: TagKind;
  color: string;
}

interface CreateTagInput {
  name: string;
  kind: TagKind;
  color: string;
}

interface ISearchTagsInput {
  kind: TagKind;

  page: number;
  per_page: number;
  order: OrderFilter;
}

type SearchTagsInput = Partial<ISearchTagsInput>;

interface TagsSearchResponse {
  total: number;
  skus: Tag[];
}

export type { Tag, CreateTagInput, SearchTagsInput, TagsSearchResponse };
