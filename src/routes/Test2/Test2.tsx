import {
  Button,
  Flex,
  Group,
  Pagination,
  Paper,
  Space,
  TextInput,
} from '@mantine/core';
import { useForm } from '@mantine/form';
import { useDisclosure, usePagination } from '@mantine/hooks';

import { useSearchSkus } from '@Service/hooks/sku';
import { OrderFilter, QueryOrder } from '@Utils/Order';
import ModalInsertSKU from './ModalUpsertSKU';
import TableLists from './TableLists';

const defaultFilter: SearchFilter<Partial<SKUSearchInput>> = {
  search: {},
  paging: {
    pageSize: 10,
    page: 0,
  },
  order: {
    field: 'updatedAt',
    orderBy: 'ASC',
  },
};

interface SearchFilter<T> {
  search?: T;
  paging?: {
    pageSize: number;
    page: number;
  };
  order?: {
    field: string;
    orderBy: 'ASC' | 'DESC';
  };
}

interface SKUSearchInput {
  productName: string;
  productType: string;
}

interface SearchResult<T> {
  total: number;
  result: T[];
}

const getData = (
  filter: SearchFilter<Partial<SKUSearchInput>>
): SearchResult<any> => {
  return { total: 10, result: [] };
};

interface TosearchFilterInput {
  search: Partial<SKUSearchInput>;
  page: number;
}

const toSearchFilter = (searchValiables: TosearchFilterInput) => {
  const { search, page } = searchValiables;
  const searchFilter: SearchFilter<Partial<SKUSearchInput>> = {
    search,
    paging: {
      pageSize: 10,
      page,
    },
    order: {
      field: 'updatedAt',
      orderBy: 'ASC',
    },
  };
  return searchFilter;
};

const Test2 = () => {
  const form = useForm({
    initialValues: {
      productName: '',
      productType: '',
    },
  });
  const [{ response }] = useSearchSkus({
    order: new OrderFilter('created_at', QueryOrder.DESC),
  });
  const [opened, { open, close }] = useDisclosure(false);
  const { active: actionPage, setPage } = usePagination({
    total: 10,
    initialPage: 1,
  });

  const clearSearch = () => {
    form.reset();
  };
  const handleSearch = () => {
    const search = { ...form.values };
    const searchFilter: SearchFilter<Partial<SKUSearchInput>> = toSearchFilter({
      search,
      page: actionPage,
    });
    const result = getData(searchFilter);
  };
  const handleChangPage = (page: number) => {
    const search = { ...form.values };
    const searchFilter: SearchFilter<Partial<SKUSearchInput>> = toSearchFilter({
      search,
      page,
    });
    setPage(page);
    const result = getData(searchFilter);
  };
  return (
    <>
      <Group position="apart">
        <h1>รายการคลังสินค้า</h1>
        <Button onClick={open}>เพิ่ม</Button>
      </Group>
      <Group position="apart">
        <form onSubmit={handleSearch}>
          <Flex
            mih={50}
            justify="flex-start"
            align="flex-end"
            direction="row"
            wrap="nowrap"
          >
            <TextInput
              placeholder="ค้นหาชื่อสินค้า"
              label="ชื่อสินค้า"
              {...form.getInputProps('productName')}
            />
            <Space w="md" />
            <TextInput
              placeholder="ค้นหาประเภทสินค้า"
              label="ประเภทสินค้า"
              {...form.getInputProps('productType')}
            />
            <Space w="md" />
            <Button type="submit">ค้นหา</Button>
            <Space w="md" />
            <Button variant="outline" onClick={clearSearch}>
              ล้างห้องน้ำ
            </Button>
          </Flex>
        </form>
      </Group>
      <Paper shadow="xs" radius="md" p="md" mt="md">
        <TableLists elements={response?.skus ?? []} />
      </Paper>
      <Flex
        mih={50}
        justify="center"
        align="center"
        direction="row"
        wrap="nowrap"
      >
        <Pagination
          value={actionPage}
          onChange={handleChangPage}
          total={10}
          size="md"
          mt="md"
        />
      </Flex>
      <ModalInsertSKU close={close} opened={opened} />
    </>
  );
};

export default Test2;
