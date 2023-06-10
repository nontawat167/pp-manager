import { Modal as MantineModal, ModalProps } from '@mantine/core';
import { PropsWithChildren } from 'react';
import { useAppConfig } from '../../config/ConfigContext';

const Modal = ({ children, ...otherProps }: PropsWithChildren<ModalProps>) => {
  const {
    modal: { zIndex },
  } = useAppConfig();
  return (
    <MantineModal {...otherProps} centered zIndex={zIndex}>
      {children}
    </MantineModal>
  );
};

export default Modal;
