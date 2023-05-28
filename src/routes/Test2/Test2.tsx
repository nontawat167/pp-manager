import { Container } from '@mantine/core';

import { useInvoker } from '@Service/hooks/ipc';
import { Sku } from '@Service/types/sku';
import TableLists from './TableLists';

// const elements = [
//   {
//     id: '12',
//     createdAt: '2023-05-27 08:55:06',
//     updateAt: '2023-05-27 08:55:06',
//     name: 'พิทักษ์พงศ์1',
//     price: 99,
//     type: 'อะไหล่อ่ะ',
//   },
//   {
//     id: '13',
//     createdAt: '2023-05-27 08:59:38',
//     updateAt: '2023-05-27 08:59:38',
//     name: 'พิทักษ์พงศ์2',
//     price: 99,
//     type: 'อะไหล่อ่ะ',
//   },
// ];

const Test2 = () => {
  const [{ response }] = useInvoker<Sku[]>('get_skus');

  return (
    <>
      <h1>รายการคลังสินค้า</h1>
      <Container
        fluid
        sx={{ backgroundColor: 'white', borderRadius: 5, width: '100%' }}
      >
        <TableLists elements={response} />
      </Container>
    </>
  );
};

export default Test2;
