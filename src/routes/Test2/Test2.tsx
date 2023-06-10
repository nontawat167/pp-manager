import { Button, Flex, Paper } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';

import { useSearchSkus } from '@Service/hooks/sku';
import { OrderFilter, QueryOrder } from '@Utils/Order';
import ModalInsertSKU from './ModalUpsertSKU';
import TableLists from './TableLists';

const Test2 = () => {
  const [{ response }] = useSearchSkus({
    order: new OrderFilter('created_at', QueryOrder.DESC),
  });
  const [opened, { open, close }] = useDisclosure(false);
  return (
    <>
      <h1>รายการคลังสินค้า</h1>

      <Flex
        mih={50}
        justify="flex-end"
        align="center"
        direction="row"
        wrap="nowrap"
      >
        <Button onClick={open}>เพิ่ม</Button>
      </Flex>
      <Paper shadow="xs" radius="md" p="md" mt="md">
        <TableLists elements={response?.skus ?? []} />
      </Paper>
      <ModalInsertSKU close={close} opened={opened} />
    </>
  );
};

export default Test2;
