import Modal from '@Components/modal/Modal';
import { Box, Button, Group, Select, TextInput } from '@mantine/core';
import { useForm } from '@mantine/form';
import { randomId } from '@mantine/hooks';
import { randomInt } from 'crypto';
import { useState } from 'react';
import { formatWithOptions } from 'util';

interface ModalInsert {
  opened: boolean;
  close: () => void;
}

const ModalInsertSKU = (param: ModalInsert) => {
  const form = useForm({
    initialValues: {
      name: '',
      price: 0,
      type: '',
    },
    validate: {
      name: (value) => (value === '' ? 'กรุณาระบุชื่อสินค้า' : null),
      type: (value) => (value === '' ? 'กรุณาระบุประเภทสินค้า' : null),
    },
  });
  const [data, setData] = useState([
    { value: 'M001', label: 'พิทักษ์พงศ์' },
    { value: 'M002', label: 'งามอ่อง' },
  ]);

  const saveProdcut = (value: {
    name: string;
    price: number;
    type: string;
  }) => {
    console.log(value);
  };
  return (
    <>
      <Modal opened={param.opened} onClose={param.close} title="เพิ่มสินค้า">
        <Box maw={320} mx="auto">
          <form onSubmit={form.onSubmit((value) => saveProdcut(value),param.close)}>
            <TextInput
              mt="md"
              label="ชื่อสินค้า"
              placeholder="ชื่อสินค้า"
              {...form.getInputProps('name')}
            />
            <TextInput
              mt="md"
              type="number"
              label="ราคาสินค้า"
              placeholder="ราคาสินค้า"
              {...form.getInputProps('price')}
            />
            <Select
              mt="md"
              label="ประเภทสินค้า"
              data={data}
              placeholder="ประเภทสินค้า"
              nothingFound="ไม่พบประเภทสินค้า"
              searchable
              creatable
              {...form.getInputProps('type')}
              getCreateLabel={(query) => `+ Create ${query}`}
              onCreate={(query) => {
                const item = { value: query, label: query };
                setData((current) => [...current, item]);
                return item;
              }}
            />
            <Group position="center" mt="xl">
              <Button type="submit" variant="outline">
                เพิ่ม
              </Button>
            </Group>
          </form>
        </Box>
      </Modal>
    </>
  );
};

export default ModalInsertSKU;
