import Modal from '@Components/modal/Modal';
import ModalFormSubmissionButton from '@Components/modal/ModalFormSubmissionButton';
import { Box } from '@mantine/core';
import { useCreateSku } from '@Service/hooks/sku';
import {
  SKUUpsertFormProvider,
  SKUUpsertFormInput,
} from './SKUUpsertFormContext';
import SKUUpertFormFields from './SKUUpsertFormFields';

interface ModalUpsert {
  opened: boolean;
  close: () => void;
  onSubmit: () => void;
}

const ModalUpsertSKU = ({ opened, close, onSubmit }: ModalUpsert) => {
  const { mutate: createSku } = useCreateSku();
  const handleSubmit = (data: SKUUpsertFormInput) => {
    const { name, price, type } = data;
    createSku({
      name,
      price: Number(price),
      product_type: type,
    });
    onSubmit();
    close();
  };

  return (
    <Modal opened={opened} onClose={close} title="เพิ่มสินค้า">
      <Box>
        <SKUUpsertFormProvider
          initValue={{ name: '', price: 0, type: '' }}
          onSubmit={handleSubmit}
        >
          <SKUUpertFormFields />
          <ModalFormSubmissionButton
            onClose={close}
            cancelText="อิอิ"
            confirmText="confirmeiei"
          />
        </SKUUpsertFormProvider>
      </Box>
    </Modal>
  );
};

export default ModalUpsertSKU;
