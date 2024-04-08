import { Query, Mutation } from '@Service/types';
import {
  CreateTagInput,
  SearchTagsInput,
  Tag,
  TagsSearchResponse,
} from '@Service/types/tag';

// export const useSearchTags = (
//   searchInput: SearchTagsInput,
//   opts?: UseInvokerOptions
// ): UseInvokerResponse<SearchTagsInput, TagsSearchResponse> => {
//   const result = useInvoker<SearchTagsInput, TagsSearchResponse>(
//     Query.SEARCH_TAGS,
//     searchInput,
//     opts
//   );

//   return result;
// };

// export const useCreateTag = (
//   createTagInput: Partial<CreateTagInput>,
//   opts?: UseInvokerOptions
// ): UseInvokerResponse<CreateTagInput, Tag> => {
//   const result = useInvoker<CreateTagInput, Tag>(
//     Mutation.CREATE_TAG,
//     createTagInput,
//     opts
//   );

//   return result;
// };
