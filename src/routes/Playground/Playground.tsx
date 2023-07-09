import Table from '@Components/table/Table';
import { Flex, Pagination } from '@mantine/core';

const data = [
  {
    id: '1',
    productName: 'test',
    productType: 'testType',
  },
  {
    id: '2',
    productName: 'test1',
    productType: 'testType1',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
  {
    id: '3',
    productName: 'test2',
    productType: 'testType2',
  },
];

const columns = [
  {
    header: 'ชื่อสินค้า',
    field: 'productName',
  },
  {
    header: 'ประเภทสินค้า',
    field: 'productType',
  },
  {
    header: 'วันที่',
    field: 'productType',
  },
];

const Playground = () => {
  return (
    <>
      <Table data={data} columns={columns} />
      <Flex
        mih={50}
        justify="center"
        align="center"
        direction="row"
        wrap="nowrap"
      >
        <Pagination
          value={1}
          onChange={() => {}}
          total={10}
          size="md"
          mt="md"
        />
      </Flex>
    </>
  );
};

export default Playground;
