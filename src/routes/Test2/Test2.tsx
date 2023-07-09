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

const Test2 = () => {
  const form = useForm({
    initialValues: {
      productName: '',
      productType: '',
    },
  });
  const [{ response }, refetchSkus] = useSearchSkus({
    order: new OrderFilter('created_at', QueryOrder.DESC),
  });
  const [opened, { open, close }] = useDisclosure(false);
  const totalRows = response?.total ?? 0;
  const rowPerPage = 10;
  const totalPage = Math.ceil(totalRows / rowPerPage);
  const { active: actionPage, setPage } = usePagination({
    total: totalPage,
    initialPage: 1,
  });

  const clearSearch = () => {
    form.reset();
  };
  const handleSearch = (event: any) => {
    event.preventDefault();
    const search = { ...form.values };
    const { productName, productType } = search;
    refetchSkus({
      name: productName !== '' ? productName : undefined,
      product_type: productType !== '' ? productType : undefined,
      order: new OrderFilter('created_at', QueryOrder.DESC),
    });
  };
  const handleChangPage = (page: number) => {
    const search = { ...form.values };
    const { productName, productType } = search;
    refetchSkus({
      name: productName !== '' ? productName : undefined,
      product_type: productType !== '' ? productType : undefined,
      order: new OrderFilter('created_at', QueryOrder.DESC),
      page,
    });
    setPage(page);
  };
  const reFetch = () => {
    refetchSkus();
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
          total={totalPage}
          size="md"
          mt="md"
        />
      </Flex>
      <ModalInsertSKU onSubmit={reFetch} close={close} opened={opened} />
    </>
  );
};

export default Test2;
